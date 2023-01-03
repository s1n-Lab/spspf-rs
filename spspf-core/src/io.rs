use psp::sys::{sceIoChdir, sceIoOpen, sceIoRead, IoOpenFlags};

pub struct File {}

impl File {
    pub fn read(path: &str, buf: &mut [u8]) {
        unsafe {
            sceIoChdir(b"ms0:/PSP/GAME/SPSPF\0".as_ptr());
            let file = sceIoOpen(path.as_ptr(), IoOpenFlags::RD_ONLY, 0o777);
            sceIoRead(file, buf.as_mut_ptr() as *mut _, buf.len() as u32);
        }
    }
}
