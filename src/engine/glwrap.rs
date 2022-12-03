use std::convert::TryInto;
use std::sync::mpsc::Receiver;
use std::ffi::{CString, c_void};
use glfw::{Glfw, Context};
use std::os::raw::c_void as os_void;
use nalgebra_glm::{TMat4, TVec3};
use crate::engine::texture::Texture;
use crate::engine::shader::Shader;
use crate::engine::vertices::MeshKit;
use crate::glad_gl::gl;

pub struct Gl {
    pub context: Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>
}

impl Gl {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw.with_primary_monitor(|c, m| {
            c.window_hint(glfw::WindowHint::ContextVersion(4, 5));
            if let Some(monitor) = m {
                let (xpos, ypos, width, height) = monitor.get_workarea();
                let (mut w, ev) = c
                    .create_window(
                        (width - xpos).try_into().unwrap(),
                        (height - ypos).try_into().unwrap(),
                        "rust-angband",
                        glfw::WindowMode::Windowed,
                    )
                    .expect("Failed to create GLFW window.");
                if let Some(mode) = monitor.get_video_mode() {
                    w.set_monitor(
                        glfw::WindowMode::Windowed,
                        0,
                        0,
                        mode.width,
                        mode.height,
                        Some(mode.refresh_rate),
                    );
                    w.maximize();
                }
                (w, ev)
            } else {
                c.create_window(800, 600, "rust-angband", glfw::WindowMode::Windowed)
                    .expect("Failed to create GLFW window.")
            }
        });

        window.set_key_polling(true);
        window.make_current();
        window.set_framebuffer_size_polling(true);
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        gl::load(|e| glfw.get_proc_address_raw(e) as * const os_void);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        Gl { 
            context: glfw,
            events,
            window
        }
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
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

    pub fn activate_texture(&self, texture: &Texture) {
        unsafe {
            // gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.handle);
        }
    }

    pub fn tick(&self) -> f64 {
        self.context.get_time()
    }

    pub fn events(&mut self) -> glfw::FlushedMessages<'_, (f64, glfw::WindowEvent)> {
        self.context.poll_events();
        glfw::flush_messages(&self.events)
    }

    pub fn window_size(&self) -> (f32, f32) {
        let (w, h) = self.window.get_framebuffer_size();
        (w as f32, h as f32)
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn close(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn swap(&mut self) {
        self.window.swap_buffers();
    }
}