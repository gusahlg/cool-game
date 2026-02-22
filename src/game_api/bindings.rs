use crate::world::levels::Level;

pub static mut LEVEL_PTR: *mut Level = std::ptr::null_mut();

pub fn bind_level(level: &mut Level) {
    unsafe { LEVEL_PTR = level as *mut Level; }
}




