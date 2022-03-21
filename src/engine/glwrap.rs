use glfw::Glfw;
use crate::glad_gl::gl::{load};
use std::os::raw::c_void as os_void;

pub struct Gl {
    pub context: Glfw
}

impl Gl {
    pub fn new(glfw: Glfw) -> Self {
        load(|e| glfw.get_proc_address_raw(e) as * const os_void);

        Gl { 
            context: glfw
        }
    }
}