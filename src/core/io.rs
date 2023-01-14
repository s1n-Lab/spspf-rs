extern crate alloc;

use alloc::{string::String, borrow::ToOwned, ffi::CString};
use psp::sys::{sceIoChdir, sceIoOpen, sceIoRead, IoOpenFlags, sceIoGetstat, SceIoStat, sceIoLseek};

pub struct FileManager {}

impl FileManager {
    pub fn new(module_name: &str) -> Result<Self, String> {
        unsafe {
            if sceIoChdir(alloc::format!("ms0:/PSP/GAME/{}\0", module_name).as_ptr()) < 0 {
                return Err("Failed to access the specified module folder.".to_owned())
            }
        }

        return Ok(Self {})
    }

    pub fn read_file(&mut self, file_name: &str) -> Result<&mut [u8], String> {
        unsafe {
            let file_stats = core::mem::MaybeUninit::<SceIoStat>::uninit().as_mut_ptr();
            if sceIoGetstat(alloc::format!("{}\0", file_name).as_ptr(), file_stats) < 0 {
                return Err("Failed to access the specified file!".to_owned())
            }

            crate::utils::stdout(alloc::format!("\nReading {} ({} bytes)", file_name, (*file_stats).st_size).as_str());

            let file = sceIoOpen(alloc::format!("{}\0", file_name).as_ptr(), IoOpenFlags::RD_ONLY, 0777);
            if file.0 < 0 {
                return Err("Failed to open the specified file!".to_owned())
            }

            let mut buffer = CString::new(String::new()).unwrap();
            if (*file_stats).st_size as i32 != sceIoRead(file, buffer.as_ptr() as *mut _, (*file_stats).st_size as u32) {
                return Err("Failed to read the specified file!".to_owned())
            }

            crate::utils::stdout(alloc::format!("\nReading {} ({} bytes)\nContent: {:x?}", file_name, (*file_stats).st_size, buffer.as_bytes_with_nul().len()).as_str());

            //return Ok(*buffer.as_mut_ptr());
            return Err("Safe".to_owned());
        }
    }
}