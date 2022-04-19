use rusttype::Font;
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
use crate::ui::cp437::Cp437;
use crate::random::Random;

pub mod shader;
pub mod vertices;
pub mod glwrap;
pub mod texture;
pub mod camera;

pub struct Engine {
    pub gl: glwrap::Gl,
    pub graphics_modes: GraphicsModeService,
    pub font: Cp437
}

impl Engine {
    pub fn new(graphics: GraphicsModeService) -> Engine {
        let content = std::fs::read(Path::new("resources/fonts/FiraCode-Medium.ttf")).unwrap();
        let face = Font::try_from_vec(content).unwrap();

        Engine {
            gl: glwrap::Gl::new(),
            graphics_modes: graphics,
            font: Cp437::from_face(&face, 255.0)
        }
    }

    pub fn run(&mut self) {
        let mut shader = shader::Shader::new(
            Path::new("resources/shaders/vertex_default.glsl"),
            Path::new("resources/shaders/fragment_default.glsl"),
        )
        .unwrap();

        let floor_vertex_data = [
            (glm::vec3( 0.5,  0.0,  0.5), glm::vec2(1.0, 1.0)),
            (glm::vec3( 0.5,  0.0, -0.5), glm::vec2(1.0, 0.0)),
            (glm::vec3(-0.5,  0.0, -0.5), glm::vec2(0.0, 0.0)),
            (glm::vec3(-0.5,  0.0,  0.5), glm::vec2(0.0, 1.0))
        ];
        let floor_indices = [
            0, 1, 3,
            1, 2, 3
        ];
        let floor_mesh = vertices::MeshKit::new(&floor_vertex_data, &floor_indices);

        let standing_vertex_data = [
            (glm::vec3( 0.5,  0.5,  0.0), glm::vec2(1.0, 1.0)),
            (glm::vec3( 0.5, -0.5,  0.0), glm::vec2(1.0, 0.0)),
            (glm::vec3(-0.5, -0.5,  0.0), glm::vec2(0.0, 0.0)),
            (glm::vec3(-0.5,  0.5,  0.0), glm::vec2(0.0, 1.0))
        ];
        let standing_indices = [
            0, 1, 3,
            1, 2, 3
        ];
        let standing_mesh = vertices::MeshKit::new(&standing_vertex_data, &standing_indices);

        let cube_vertex_data = [
                (glm::vec3(-0.5, -0.5, -0.5), glm::vec2(0.0, 0.0)),
                (glm::vec3( 0.5, -0.5, -0.5), glm::vec2(1.0, 0.0)),
                (glm::vec3( 0.5,  0.5, -0.5), glm::vec2(1.0, 1.0)),
                (glm::vec3(-0.5,  0.5, -0.5), glm::vec2(0.0, 1.0)),
                (glm::vec3(-0.5, -0.5,  0.5), glm::vec2(0.0, 0.0)),
                (glm::vec3( 0.5, -0.5,  0.5), glm::vec2(1.0, 0.0)),
                (glm::vec3( 0.5,  0.5,  0.5), glm::vec2(1.0, 1.0)),
                (glm::vec3(-0.5,  0.5,  0.5), glm::vec2(0.0, 1.0)),
                (glm::vec3(-0.5,  0.5,  0.5), glm::vec2(1.0, 0.0)),
                (glm::vec3(-0.5,  0.5, -0.5), glm::vec2(1.0, 1.0)),
                (glm::vec3(-0.5, -0.5, -0.5), glm::vec2(0.0, 1.0)),
                (glm::vec3( 0.5,  0.5,  0.5), glm::vec2(1.0, 0.0)),
                (glm::vec3( 0.5, -0.5, -0.5), glm::vec2(0.0, 1.0)),
                (glm::vec3( 0.5, -0.5,  0.5), glm::vec2(0.0, 0.0)),
                (glm::vec3( 0.5, -0.5, -0.5), glm::vec2(1.0, 1.0)),
                (glm::vec3(-0.5,  0.5,  0.5), glm::vec2(0.0, 0.0)),
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

        let texture = texture::Texture::from_file(
            Path::new("resources/images/container.jpg")
        );

        let (w, h) = self.gl.window_size();
        
        let identity = glm::identity::<f32, 4>();

        let (grid_width, grid_height) = (250, 250);

        let colors = crate::colors::ColorService::new();

        let mut rng = Random::new();
        let mut grid = vec![vec![('.', colors.angband_color_table[&crate::colors::Colors::White], colors.angband_color_table[&crate::colors::Colors::Dark]); grid_width]; grid_height];

        let mut camera = camera::Camera::offset(20.0, 30.0, 10.0, 0.0, -1.0, -0.01);
        // camera.mode = camera::CameraMode::Offset (glm::vec3(0.0, -1.0, -0.01));

        let projection = glm::perspective(w / h, glm::radians(&glm::vec1(45.0)).x, 0.1, 100.0);
        // let projection = glm::ortho(0.0, w, 0.0, h, 0.1, 100.0);

        let mut prev: f64 = 0.0;
        let mut frame_count = 0;
        while !self.gl.should_close() {
            let total_elapsed = self.gl.tick();
            let last_frame = total_elapsed - prev;
            prev = total_elapsed;
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
                    WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                        let font = &self.font;
                        for row in grid.iter_mut() {
                            for entry in row.iter_mut() {
                                entry.0 = font.random(&mut rng);
                                entry.1 = colors.angband_color_table[&colors.random(&mut rng)];
                                entry.2 = colors.angband_color_table[&colors.random(&mut rng)];
                            }
                        }
                        // grid = (0..grid_height).map(|_| {
                        //     (0..grid_width).map(|_| {(font.random(&mut rng), colors.angband_color_table[&colors.random(&mut rng)], colors.angband_color_table[&colors.random(&mut rng)])}).collect()
                        // }).collect();
                    }
                    _ => (),
                }
            }

            if close {
                self.gl.close();
            }

            self.gl.clear_color(0.2, 0.3, 0.3, 1.0);
            shader.activate();

            let mut drawn = 0;
            for (y, r) in grid.iter().enumerate() {
                for (x, (ch, fg, bg)) in r.iter().enumerate() {
                    if (y as f32) < camera.position.y + 30.0 && (y as f32) > camera.position.y - 30.0
                        && (x as f32) < camera.position.x + 45.0 && (x as f32) > camera.position.x - 45.0 {
                            let model = glm::translate(&identity, &glm::vec3(x as f32, 0.0, y as f32));
            
                            shader.matrix_parameter("model", &model);
                            shader.matrix_parameter("view", &camera.view);
                            shader.matrix_parameter("projection", &projection);
        
                            shader.vector_parameter("fgColor", fg);
                            shader.vector_parameter("bgColor", bg);
            
                            self.gl.activate_texture(self.font.char(*ch));
                            self.gl.render_mesh(&floor_mesh);
                            drawn += 1;
                    }
                }
            }

            self.gl.swap();
            frame_count += 1;
            if frame_count % 5 == 0 {
                println!("FPS ~ {}, {} drawn", 1.0 / last_frame, drawn);
            }
        }
    }
}
