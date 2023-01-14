// Based on PSPSDK's pspaudiolib
//
// Copyright (c) 2005  adresd
// Copyright (c) 2005  Marcus R. Brown
// Copyright (c) 2005  James Forshaw
// Copyright (c) 2005  John Kelley
// Copyright (c) 2005  Jesper Svennevid
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. The names of the authors may not be used to endorse or promote products
//    derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHORS ``AS IS'' AND ANY EXPRESS OR
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
// OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
// THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

extern crate alloc;
use alloc::{
    format,
    string::{String, ToString},
};
use core::{ffi::c_void, ptr};
use psp::sys::{
    sceAudioChRelease, sceAudioChReserve, sceAudioGetChannelRestLen, sceAudioOutputPanned,
    sceAudioSetChannelDataLen, sceKernelCreateThread, sceKernelDeleteThread,
    sceKernelExitDeleteThread, sceKernelGetThreadId, sceKernelStartThread, AudioFormat, SceUid,
    ThreadAttributes, AUDIO_CHANNEL_MAX, AUDIO_SAMPLE_MAX, AUDIO_VOLUME_MAX,
};
use crate::core::utils;

static mut AUDIO_STATUS: [AudioChannelInfo; AUDIO_CHANNEL_MAX as usize + 1] = [AudioChannelInfo {
    handle: -1,
    thread_handle: -1,
    volume_left: AUDIO_VOLUME_MAX as i32,
    volume_right: AUDIO_VOLUME_MAX as i32,
    active_clip: None,
    active_clip_size: None,
};
    AUDIO_CHANNEL_MAX as usize + 1];

static mut AUDIO_TERMINATE: bool = false;
static mut AUDIO_READY: bool = false;

#[derive(Clone, Copy)]
struct AudioChannelInfo {
    thread_handle: i32,
    handle: i32,
    volume_left: i32,
    volume_right: i32,
    active_clip: Option<*mut u8>,
    active_clip_size: Option<usize>,
}

pub struct Sound<const N: usize> {
    clip: &'static [u8; N],
    volume_left: i32,
    volume_right: i32,
    channel: i32,
}

pub struct AudioManager {}
impl AudioManager {
    pub fn init() -> Result<AudioManager, String> {
        // Reserve audio channels
        unsafe {
            let mut failed = false;
            for i in 0..AUDIO_CHANNEL_MAX as usize {
                AUDIO_STATUS[i].handle =
                    sceAudioChReserve(i as i32, AUDIO_SAMPLE_MAX as i32, AudioFormat::Stereo);
                if AUDIO_STATUS[i].handle < 0 {
                    failed = true;
                }
            }

            if failed {
                for i in 0..AUDIO_CHANNEL_MAX as usize {
                    if AUDIO_STATUS[i].handle != -1 {
                        sceAudioChRelease(AUDIO_STATUS[i].handle);
                    }
                }
                return Err("\n[MAINT] FATAL: Failed to reserve audio channels!".to_string());
            }
        }

        // Reserve PSP threads
        unsafe {
            let mut failed = false;
            for i in 0..AUDIO_CHANNEL_MAX as usize {
                let title = format!("audiot{}", i.clone());
                AUDIO_STATUS[i].thread_handle = sceKernelCreateThread(
                    title.as_ptr(),
                    audio_manager_thread,
                    45,
                    0x01000,
                    ThreadAttributes::empty(),
                    ptr::null_mut(),
                )
                .0;
                if AUDIO_STATUS[i].thread_handle < 0 {
                    AUDIO_STATUS[i].thread_handle = -1;
                    failed = true;
                    break;
                }

                let status =
                    sceKernelStartThread(SceUid(AUDIO_STATUS[i].thread_handle), 0, ptr::null_mut());
                if status != 0 {
                    failed = true;
                    break;
                }
            }

            if failed {
                for i in 0..AUDIO_CHANNEL_MAX as usize {
                    if AUDIO_STATUS[i].thread_handle != -1 {
                        sceKernelDeleteThread(SceUid(AUDIO_STATUS[i].thread_handle));
                    }
                    AUDIO_STATUS[i].thread_handle = -1;
                }
                return Err("\n[MAINT] FATAL: Failed to create or start threads!".to_string());
            }
        }

        Ok(AudioManager {})
    }

    pub fn end(&mut self) {
        unsafe {
            for i in 0..AUDIO_CHANNEL_MAX as usize {
                if AUDIO_STATUS[i].thread_handle != -1 {
                    sceKernelDeleteThread(SceUid(AUDIO_STATUS[i].thread_handle));
                }
                AUDIO_STATUS[i].thread_handle = -1;
            }

            for i in 0..AUDIO_CHANNEL_MAX as usize {
                if AUDIO_STATUS[i].thread_handle != -1 {
                    sceAudioChRelease(AUDIO_STATUS[i].handle);
                }
                AUDIO_STATUS[i].thread_handle = -1;
            }
        }
    }

    pub fn get_available_channel(&mut self) -> Option<i32> {
        unsafe {
            for i in 0..AUDIO_CHANNEL_MAX as usize {
                if AUDIO_STATUS[i].active_clip.is_none() && AUDIO_STATUS[i].thread_handle != -1 {
                    return Some(i as i32);
                }
            }
        }
        return None;
    }
}

unsafe extern "C" fn audio_manager_thread(_args: usize, _argp: *mut c_void) -> i32 {
    let thread_handle = sceKernelGetThreadId();
    let mut channel = -1;

    for aci in AUDIO_STATUS {
        if aci.thread_handle == thread_handle {
            channel = aci.handle;
        }
    }

    let mut channel_info;
    match channel {
        -1 => {
            utils::stdout(
                format!(
                    "\n[AUDIOC? (T{})] FATAL: Failed to find relative channel!",
                    thread_handle
                )
                .as_str(),
            );
            sceKernelExitDeleteThread(-1);
            return -1;
        }
        _ => {}
    };

    while !AUDIO_TERMINATE {
        channel_info = AUDIO_STATUS[channel as usize];
        match channel_info.active_clip {
            Some(clip) => {
                let mut start_pos: usize = 0;
                let mut restlen = 0;

                utils::stdout(
                    format!(
                        "\n[AUDIOC{:?} (T{:?})] data: {:?}, size: {:?}",
                        channel_info.handle,
                        thread_handle,
                        clip,
                        channel_info.active_clip_size.unwrap()
                    )
                    .as_str(),
                );

                while (start_pos + AUDIO_SAMPLE_MAX as usize * 4)
                    < channel_info.active_clip_size.unwrap()
                {
                    if restlen >= 0 {
                        utils::stdout(
                            format!(
                                "\n[AUDIOC{:?} (T{:?})] start_pos: {}, clip_len given: {}",
                                channel_info.handle,
                                thread_handle,
                                start_pos,
                                channel_info.active_clip_size.unwrap()
                            )
                            .as_str(),
                        );

                        let result = sceAudioOutputPanned(
                            channel_info.handle,
                            channel_info.volume_left,
                            channel_info.volume_right,
                            channel_info.active_clip.unwrap().add(start_pos) as *mut c_void,
                        );

                        if result >= 0 {
                            utils::stdout(
                                format!(
                                    "\n[AUDIOC{:?} (T{:?})] OK: Managed to output audio!",
                                    channel_info.handle, thread_handle
                                )
                                .as_str(),
                            );
                        } else {
                            utils::stdout(
                                format!(
                                    "\n[AUDIOC{:?} (T{:?})] FATAL: Failed to output audio!",
                                    channel_info.handle, thread_handle
                                )
                                .as_str(),
                            );
                            break;
                        }
                        start_pos += AUDIO_SAMPLE_MAX as usize * 4;
                    }
                    restlen = sceAudioGetChannelRestLen(channel_info.handle);
                }
                let remainder: i32 = (((channel_info.active_clip_size.unwrap()
                    % (AUDIO_SAMPLE_MAX as usize * 4)
                    / 4)
                    + 63)
                    & !63) as i32;
                sceAudioSetChannelDataLen(channel_info.handle, remainder);
                AUDIO_STATUS[channel as usize].active_clip = None;
                AUDIO_STATUS[channel as usize].active_clip_size = None;
            }
            None => {}
        }
    }

    sceKernelExitDeleteThread(0);
    0
}

impl<const N: usize> Sound<N> {
    pub fn new(clip: &'static [u8; N]) -> Sound<N> {
        Sound {
            clip,
            volume_left: AUDIO_VOLUME_MAX as i32,
            volume_right: AUDIO_VOLUME_MAX as i32,
            channel: -1,
        }
    }

    pub fn play(&mut self, audio_manager: &mut AudioManager) -> Result<i32, String> {
        let channel = match audio_manager.get_available_channel() {
            Some(r) => r as usize,
            None => return Err("\n[MAINT] ERROR: No available channels!".to_string()),
        };

        utils::stdout(
            format!(
                "\n[MAINT] data: {:?}, size: {:?}",
                self.clip.as_ptr() as *mut u8,
                self.clip.len()
            )
            .as_str(),
        );

        unsafe {
            AUDIO_STATUS[channel as usize].active_clip = Some(self.clip.as_ptr() as *mut u8);
            AUDIO_STATUS[channel as usize].active_clip_size = Some(self.clip.len());
        }

        Ok(channel as i32)
    }

    pub fn stop(&mut self) {
        todo!("Not implemented!")
    }
}
