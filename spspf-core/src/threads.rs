use core::{self, ffi::c_void, ptr};

use psp::sys::{sceKernelCreateThread, sceKernelStartThread, ThreadAttributes};

#[allow(dead_code)]
pub struct Thread<T> {
    name: *const u8,
    func: unsafe extern "C" fn(usize, *mut c_void) -> i32,
    args: *mut T,
    args_size: usize,
}

impl<T> Thread<T> {
    pub fn start_new(
        name: &str,
        func: unsafe extern "C" fn(usize, *mut c_void) -> i32,
        args: *mut T,
    ) -> i32 {
        let id = unsafe {
            sceKernelCreateThread(
                name.as_ptr(),
                func,
                33,
                0x1000,
                ThreadAttributes::empty(),
                ptr::null_mut(),
            )
        };
        unsafe {
            sceKernelStartThread(id, core::mem::size_of::<T>(), args as *mut c_void);
        }
        return id.0;
    }
}
