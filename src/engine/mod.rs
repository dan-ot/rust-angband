use std::ffi::CString;
use glfw::{Action, Context, Glfw, Key, Modifiers, Window, WindowEvent, WindowHint, WindowMode};
use std::convert::TryInto;
use std::ffi::c_void;
use std::path::Path;
use std::sync::mpsc::Receiver;

use nalgebra_glm as glm;

// use crate::ui::FontAtlas;
use crate::glad_gl::gl;
use crate::ui::graphics::GraphicsModeService;

pub mod shader;
pub mod vertices;
pub mod glwrap;
pub mod texture;
pub mod camera;

pub struct Engine {
    pub gl: glwrap::Gl,
    pub graphics_modes: GraphicsModeService,
}

impl Engine {
    pub fn new(mut context: Glfw, graphics: GraphicsModeService) -> Engine {

        // let atlas = Box::new(FontAtlas::render_atlas(&graphics.fonts[graphics.current_font], &font_context, &mut canvas, &texture_creator));

        Engine {
            gl: glwrap::Gl::new(),
            graphics_modes: graphics,
        }
    }

    pub fn run(&mut self) {
        let shader = shader::Shader::new(
            Path::new("resources/shaders/vertex_default.glsl"),
            Path::new("resources/shaders/fragment_default.glsl"),
        )
        .unwrap();

        let floor_vertex_data = [
            (glm::vec3( 0.5,  0.0,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)),
            (glm::vec3( 0.5,  0.0, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 0.0)),
            (glm::vec3(-0.5,  0.0, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 0.0)),
            (glm::vec3(-0.5,  0.0,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 1.0))
        ];
        let floor_indices = [
            0, 1, 3,
            1, 2, 3
        ];
        let floor_mesh = vertices::MeshKit::new(&floor_vertex_data, &floor_indices);

        let standing_vertex_data = [
            (glm::vec3( 0.5,  0.5,  0.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)),
            (glm::vec3( 0.5, -0.5,  0.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 0.0)),
            (glm::vec3(-0.5, -0.5,  0.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 0.0)),
            (glm::vec3(-0.5,  0.5,  0.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 1.0))
        ];
        let standing_indices = [
            0, 1, 3,
            1, 2, 3
        ];
        let standing_mesh = vertices::MeshKit::new(&standing_vertex_data, &standing_indices);

        let cube_vertex_data = [
                (glm::vec3(-0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 0.0)),
                (glm::vec3( 0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 0.0)),
                (glm::vec3( 0.5,  0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)),
                (glm::vec3(-0.5,  0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 1.0)),
                (glm::vec3(-0.5, -0.5,  0.5), glm::vec3(1.0, 0.0, 1.0), glm::vec2(0.0, 0.0)),
                (glm::vec3( 0.5, -0.5,  0.5), glm::vec3(1.0, 0.0, 1.0), glm::vec2(1.0, 0.0)),
                (glm::vec3( 0.5,  0.5,  0.5), glm::vec3(1.0, 0.0, 1.0), glm::vec2(1.0, 1.0)),
                (glm::vec3(-0.5,  0.5,  0.5), glm::vec3(1.0, 0.0, 1.0), glm::vec2(0.0, 1.0)),
                (glm::vec3(-0.5,  0.5,  0.5), glm::vec3(1.0, 1.0, 0.0), glm::vec2(1.0, 0.0)),
                (glm::vec3(-0.5,  0.5, -0.5), glm::vec3(1.0, 1.0, 0.0), glm::vec2(1.0, 1.0)),
                (glm::vec3(-0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 0.0), glm::vec2(0.0, 1.0)),
                (glm::vec3( 0.5,  0.5,  0.5), glm::vec3(0.0, 1.0, 1.0), glm::vec2(1.0, 0.0)),
                (glm::vec3( 0.5, -0.5, -0.5), glm::vec3(0.0, 1.0, 1.0), glm::vec2(0.0, 1.0)),
                (glm::vec3( 0.5, -0.5,  0.5), glm::vec3(0.0, 1.0, 1.0), glm::vec2(0.0, 0.0)),
                (glm::vec3( 0.5, -0.5, -0.5), glm::vec3(0.0, 0.0, 1.0), glm::vec2(1.0, 1.0)),
                (glm::vec3(-0.5,  0.5,  0.5), glm::vec3(0.0, 0.0, 1.0), glm::vec2(0.0, 0.0)),
        ];
        let cube_indices = [
            0, 1, 2,
            2, 3, 0,
            4, 5, 6,
            6, 7, 4,
            8, 9, 10,
            10, 4, 8,
            11, 2, 12,
            12, 13, 11,
            10, 14, 5,
            5, 4, 10,
            3, 2, 11,
            11, 15, 3
        ];
        let cube_mesh = vertices::MeshKit::new(&cube_vertex_data, &cube_indices);

        let texture = texture::Texture::new(
            Path::new("resources/images/container.jpg")
        );

        let (w, h) = self.gl.window_size();
        
        let model = glm::identity::<f32, 4>();

        let mut camera = camera::Camera::at(0.0, 30.0, 0.0);
        camera.mode = camera::CameraMode::Offset (glm::vec3(0.0, -1.0, -0.01));

        let projection = glm::perspective(w / h, glm::radians(&glm::vec1(45.0)).x, 0.1, 100.0);

        while !self.gl.should_close() {
            let elapsed = self.gl.tick();
            let events = self.gl.events();
            let mut close = false;
            for (_, event) in events {
                // This is only single until we get the rest of input working
                #[allow(clippy::single_match)]
                match event {
                    WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        close = true;
                    },
                    WindowEvent::Key(Key::W, _, Action::Release, Modifiers::Shift) => {
                        camera.step(-10.0);
                    },
                    WindowEvent::Key(Key::W, _, Action::Release, _) => {
                        camera.step(-1.0);
                    },
                    WindowEvent::Key(Key::S, _, Action::Release, Modifiers::Shift) => {
                        camera.step(10.0);
                    },
                    WindowEvent::Key(Key::S, _, Action::Release, _) => {
                        camera.step(1.0);
                    },
                    WindowEvent::Key(Key::D, _, Action::Release, Modifiers::Shift) => {
                        camera.strafe(-10.0);
                    },
                    WindowEvent::Key(Key::D, _, Action::Release, _) => {
                        camera.strafe(-1.0);
                    },
                    WindowEvent::Key(Key::A, _, Action::Release, Modifiers::Shift) => {
                        camera.strafe(10.0);
                    },
                    WindowEvent::Key(Key::A, _, Action::Release, _) => {
                        camera.strafe(1.0);
                    },
                    _ => (),
                }
            }

            if close {
                self.gl.close();
            }

            self.gl.clear_color(0.2, 0.3, 0.3, 1.0);
            self.gl.activate_shader(&shader);

            self.gl.specify_matrix_parameter(&shader, "model", &model);
            self.gl.specify_matrix_parameter(&shader, "view", &camera.view());
            self.gl.specify_matrix_parameter(&shader, "projection", &projection);

            self.gl.activate_texture(&texture);
            self.gl.render_mesh(&floor_mesh);

            self.gl.swap();
        }
    }
}
