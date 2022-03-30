use crate::engine::texture::Texture;
use crate::engine::shader::Shader;
use glfw::Glfw;
use std::os::raw::c_void as os_void;
use std::ffi::c_void;
use crate::engine::vertices::MeshKit;
use crate::glad_gl::gl;

pub struct Gl {
    pub context: Glfw
}

impl Gl {
    pub fn new(glfw: Glfw) -> Self {
        gl::load(|e| glfw.get_proc_address_raw(e) as * const os_void);

        Gl { 
            context: glfw
        }
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn render_mesh(&self, mesh: &MeshKit) {
        unsafe {
            gl::BindVertexArray(mesh.vertex_control_handle);
            // In this case, we mean '0 cast to Any', not 'pointer to memory 0'
            #[allow(clippy::zero_ptr)]
            gl::DrawElements(
                gl::TRIANGLES,
                mesh.size,
                gl::UNSIGNED_INT,
                0 as *const c_void
            );
            gl::BindVertexArray(0);
        }
    }

    pub fn activate_shader(&self, shader: &Shader) {
        unsafe {
            gl::UseProgram(shader.id);
        }
    }

    pub fn activate_texture(&self, texture: &Texture) {
        unsafe {
            // gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.handle);
        }
    }
}