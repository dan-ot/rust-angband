use rusttype::Font;
use std::ffi::CString;
use glfw::{Action, Context, Glfw, Key, Modifiers, Window, WindowEvent, WindowHint, WindowMode};
use std::convert::TryInto;
use std::ffi::c_void;
use std::path::Path;
use std::sync::mpsc::Receiver;

use nalgebra_glm as glm;

use crate::colors::Colors;
// use crate::ui::FontAtlas;
// use crate::glad_gl::gl;
use crate::ui::graphics::GraphicsModeService;
use crate::ui::tileset::Tileset;
use crate::ui::chars::Charmap;
use crate::random::Random;

pub mod shader;
pub mod vertices;
pub mod glwrap;
pub mod texture;
pub mod camera;

pub struct Engine {
    pub gl: glwrap::Gl,
    pub graphics_modes: GraphicsModeService,
    pub tiles: Tileset,
    pub chars: Charmap
}

impl Engine {
    pub fn new(graphics: GraphicsModeService) -> Engine {
        let content = std::fs::read(Path::new("resources/fonts/FiraCode-Medium.ttf")).unwrap();
        let face = Font::try_from_vec(content).unwrap();

        let fon_content = std::fs::read(Path::new("resources/fonts/8x8x.fon")).unwrap();

        let loaded_fonts = crate::ui::fon::load_fonts(&fon_content).unwrap();

        Engine {
            gl: glwrap::Gl::new(),
            graphics_modes: graphics,
            tiles: Tileset::from_cp1252(loaded_fonts.first().unwrap().to_vec(), 256),
            chars: Charmap::from_font(&face, 256.0)
        }
    }

    pub fn run(&mut self) {
        let mut playfield_shader = shader::Shader::new(
            Path::new("resources/shaders/vertex_world.glsl"),
            Path::new("resources/shaders/fragment_world.glsl"),
        )
        .unwrap();

        let mut text_shader = shader::Shader::new(
            Path::new("resources/shaders/vertex_text.glsl"),
            Path::new("resources/shaders/fragment_text.glsl")
        )
        .unwrap();

        // let floor_mesh = vertices::MeshKit::quad_flat(0.5, -0.5, -0.5, 0.5);
        // let charmap_mesh = vertices::MeshKit::quad_flat(glm::vec2(0.0, 0.0), glm::vec2(self.chars.atlas.size.0 as f32 / 60.0, self.chars.atlas.size.1 as f32 / 60.0));
        // let standing_mesh = vertices::MeshKit::quad_standing(glm::vec2(0.5, -0.5), glm::vec2(-0.5, 0.5));
        let floor_mesh = vertices::MeshKit::boxy(glm::vec2(-0.5, 0.5), glm::vec2(-0.5, 0.5), glm::vec2(-0.5, 0.5));

        let line_of_text = self.chars.line("any performance impacts when we go for a much longer line of text?");
        let (w, h) = self.gl.window_size();
        
        let identity = glm::identity::<f32, 4>();

        let (grid_width, grid_height) = (250, 250);

        let colors = crate::colors::ColorService::new();

        let mut rng = Random::new();
        let mut grid = vec![vec![('e', colors.angband_color_table[&crate::colors::Colors::White], colors.angband_color_table[&crate::colors::Colors::Dark]); grid_width]; grid_height];

        let mut camera = camera::Camera::offset(0.0, 20.0, 0.0, 0.0, -1.0, -0.01);

        let zoom = 40.0;
        let (size_x, size_y) = (w / zoom + 1.0, h / zoom + 1.0);
        println!("{} by {} at ({}, {}) - ({}, {}) to ({}, {})", size_x, size_y, camera.position.x, camera.position.z, camera.position.x - size_x, camera.position.z - size_y, camera.position.x + size_x, camera.position.z + size_y);
        let perspective = glm::perspective(w / h, glm::radians(&glm::vec1(45.0)).x, 0.1, 100.0);
        let ortho = glm::ortho(0.0, w / zoom, 0.0, h / zoom, -100.0, 100.0);

        // let mut prev: f64 = 0.0;
        // let mut frame_count = 0;
        while !self.gl.should_close() {
            // let total_elapsed = self.gl.tick();
            // let last_frame = total_elapsed - prev;
            // prev = total_elapsed;
            let events = self.gl.events();
            let mut close = false;
            for (_, event) in events {
                match event {
                    WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        close = true;
                    },
                    WindowEvent::Key(Key::W, _, Action::Release, Modifiers::Shift) => {
                        camera.step(-10.0);
                        println!("{} by {} at ({}, {}) - ({}, {}) to ({}, {})", size_x, size_y, camera.position.x, camera.position.z, camera.position.x - size_x, camera.position.z - size_y, camera.position.x + size_x, camera.position.z + size_y);
                    },
                    WindowEvent::Key(Key::W, _, Action::Release, _) => {
                        camera.step(-1.0);
                        println!("{} by {} at ({}, {}) - ({}, {}) to ({}, {})", size_x, size_y, camera.position.x, camera.position.z, camera.position.x - size_x, camera.position.z - size_y, camera.position.x + size_x, camera.position.z + size_y);
                    },
                    WindowEvent::Key(Key::S, _, Action::Release, Modifiers::Shift) => {
                        camera.step(10.0);
                        println!("{} by {} at ({}, {}) - ({}, {}) to ({}, {})", size_x, size_y, camera.position.x, camera.position.z, camera.position.x - size_x, camera.position.z - size_y, camera.position.x + size_x, camera.position.z + size_y);
                    },
                    WindowEvent::Key(Key::S, _, Action::Release, _) => {
                        camera.step(1.0);
                        println!("{} by {} at ({}, {}) - ({}, {}) to ({}, {})", size_x, size_y, camera.position.x, camera.position.z, camera.position.x - size_x, camera.position.z - size_y, camera.position.x + size_x, camera.position.z + size_y);
                    },
                    WindowEvent::Key(Key::D, _, Action::Release, Modifiers::Shift) => {
                        camera.strafe(-10.0);
                        println!("{} by {} at ({}, {}) - ({}, {}) to ({}, {})", size_x, size_y, camera.position.x, camera.position.z, camera.position.x - size_x, camera.position.z - size_y, camera.position.x + size_x, camera.position.z + size_y);
                    },
                    WindowEvent::Key(Key::D, _, Action::Release, _) => {
                        camera.strafe(-1.0);
                        println!("{} by {} at ({}, {}) - ({}, {}) to ({}, {})", size_x, size_y, camera.position.x, camera.position.z, camera.position.x - size_x, camera.position.z - size_y, camera.position.x + size_x, camera.position.z + size_y);
                    },
                    WindowEvent::Key(Key::A, _, Action::Release, Modifiers::Shift) => {
                        camera.strafe(10.0);
                        println!("{} by {} at ({}, {}) - ({}, {}) to ({}, {})", size_x, size_y, camera.position.x, camera.position.z, camera.position.x - size_x, camera.position.z - size_y, camera.position.x + size_x, camera.position.z + size_y);
                    },
                    WindowEvent::Key(Key::A, _, Action::Release, _) => {
                        camera.strafe(1.0);
                        println!("{} by {} at ({}, {}) - ({}, {}) to ({}, {})", size_x, size_y, camera.position.x, camera.position.z, camera.position.x - size_x, camera.position.z - size_y, camera.position.x + size_x, camera.position.z + size_y);
                    },
                    WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                        println!("Grid swap!");
                        let font = &self.tiles;
                        for row in grid.iter_mut() {
                            for entry in row.iter_mut() {
                                entry.0 = font.random(&mut rng);
                                entry.1 = colors.angband_color_table[&colors.random(&mut rng)];
                                entry.2 = colors.angband_color_table[&colors.random(&mut rng)];
                            }
                        }
                    }
                    _ => (),
                }
            }

            if close {
                self.gl.close();
            }

            self.gl.clear_color(0.2, 0.3, 0.3, 1.0);
            playfield_shader.activate();

            // let mut drawn = 0;
            for (y, r) in grid.iter().enumerate() {
                for (x, (ch, fg, bg)) in r.iter().enumerate() {
                    if (y as f32) < camera.position.z + size_y && (y as f32) > camera.position.z - size_y
                        && (x as f32) < camera.position.x + size_x && (x as f32) > camera.position.x - size_x {
                            let model = glm::translate(&identity, &glm::vec3(x as f32, 0.0, y as f32));
            
                            playfield_shader.matrix_parameter("model", &model);
                            playfield_shader.matrix_parameter("view", &camera.view);
                            playfield_shader.matrix_parameter("projection", &perspective);
        
                            playfield_shader.vector_parameter("fgColor", fg);
                            playfield_shader.vector_parameter("bgColor", bg);
                            playfield_shader.float_parameter("light", (rng.damroll(1, 5) - 1) as f32);
            
                            self.gl.activate_texture(self.tiles.char(*ch));
                            self.gl.render_mesh(&floor_mesh);
                            // drawn += 1;
                    }
                }
            }

            let text_model = glm::translate(&identity, &glm::vec3(10.0, 0.0, -10.0));
            text_shader.activate();
            text_shader.matrix_parameter("model", &text_model);
            text_shader.matrix_parameter("view", &camera.view);
            text_shader.matrix_parameter("projection", &ortho);

            text_shader.vector_parameter("fgColor", &colors.angband_color_table[&Colors::White]);

            self.gl.activate_texture(line_of_text.texture);
            self.gl.render_mesh(&line_of_text.renderable);

            self.gl.swap();
            // frame_count += 1;
            // if frame_count % 5 == 0 {
            //     println!("FPS ~ {}, {} drawn", 1.0 / last_frame, drawn);
            // }
        }
    }
}
