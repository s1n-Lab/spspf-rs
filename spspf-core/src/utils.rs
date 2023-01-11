extern crate alloc;
use core::ops::Range;

use alloc::{string::String, borrow::ToOwned};
use psp::sys::{
    sceIoWrite, sceKernelStdout, sceKernelUtilsMt19937Init, sceKernelUtilsMt19937UInt,
    sceRtcGetCurrentClockLocalTime, SceKernelUtilsMt19937Context, ScePspDateTime,
};

/// Prints a message to the STDOUT, accessible via PPSSPP's debug console or via PSPLink with real hardware.
pub fn stdout(message: &str) {
    unsafe {
        sceIoWrite(sceKernelStdout(), message.as_ptr() as *mut _, message.len());
    }
}

// Based on (rand's uniform distribution implementation)[https://docs.rs/rand/latest/src/rand/distributions/uniform.rs.html]
//
// Copyright 2018-2020 Developers of the Rand project.
// Copyright 2017 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
/// Returns a random `i32` inside the provided `Range<i32>` using the PSP's MT19937 and uniform distribuition.
pub fn rnd_range(range: Range<i32>) -> Result<i32, String> {
    let n_range = range.end.wrapping_sub(range.start).wrapping_add(1) as u32;
    let zone = (n_range << n_range.leading_zeros()).wrapping_sub(1);

    unsafe {
        let mt_context: *mut SceKernelUtilsMt19937Context =
            core::mem::MaybeUninit::<SceKernelUtilsMt19937Context>::uninit().as_mut_ptr();
        let current_time = core::mem::MaybeUninit::<ScePspDateTime>::uninit().as_mut_ptr();

        loop {
            if sceRtcGetCurrentClockLocalTime(current_time) < 0 {
                return Err("Failed on sceRtcGetCurrentClockLocalTime.".to_owned());
            }
            if sceKernelUtilsMt19937Init(mt_context, (*current_time).microseconds) < 0 {
                return Err("Failed on sceKernelUtilsMt19937Init.".to_owned());
            }
            let v = sceKernelUtilsMt19937UInt(mt_context);

            let (_hi, lo) = v.widening_mul(n_range);
            if lo <= zone {
                return Ok(range.start.wrapping_add(lo as i32));
            }
        }
    }
}
