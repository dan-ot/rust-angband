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

pub struct Engine {
    pub gl: glwrap::Gl,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
    pub graphics_modes: GraphicsModeService,
}

impl Engine {
    pub fn new(mut context: Glfw, graphics: GraphicsModeService) -> Engine {
        let (mut window, events) = context.with_primary_monitor(|c, m| {
            c.window_hint(WindowHint::ContextVersion(3, 3));
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
                        WindowMode::Windowed,
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
                c.create_window(800, 600, "rust-angband", WindowMode::Windowed)
                    .expect("Failed to create GLFW window.")
            }
        });

        window.set_key_polling(true);
        window.make_current();
        window.set_framebuffer_size_polling(true);
        context.set_swap_interval(glfw::SwapInterval::Sync(1));

        // let atlas = Box::new(FontAtlas::render_atlas(&graphics.fonts[graphics.current_font], &font_context, &mut canvas, &texture_creator));

        Engine {
            gl: glwrap::Gl::new(context),
            window,
            events,
            graphics_modes: graphics,
        }
    }

    pub fn run(&mut self) {
        let shader = shader::Shader::new(
            Path::new("resources/shaders/vertex_default.glsl"),
            Path::new("resources/shaders/fragment_default.glsl"),
        )
        .unwrap();

        let flat_vertex_data = [
            (glm::vec3( 0.5,  0.5, 0.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)),
            (glm::vec3( 0.5, -0.5, 0.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 0.0)),
            (glm::vec3(-0.5, -0.5, 0.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 0.0)),
            (glm::vec3(-0.5,  0.5, 0.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 1.0))
        ];
        let flat_indices = [
            0, 1, 3,
            1, 2, 3
        ];
        let flat_mesh = vertices::MeshKit::new(&flat_vertex_data, &flat_indices);

        let cube_vertex_data = [
                (glm::vec3(-0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 0.0)), // 0
                (glm::vec3( 0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 0.0)), // 1
                (glm::vec3( 0.5,  0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)), // 2
           // 2  0.5,  0.5, -0.5,  1.0, 1.0,
                (glm::vec3(-0.5,  0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 1.0)), // 3
           // 0 -0.5, -0.5, -0.5,  0.0, 0.0
            
                (glm::vec3(-0.5, -0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 0.0)), // 4
                (glm::vec3( 0.5, -0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 0.0)), // 5
                (glm::vec3( 0.5,  0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)), // 6
           // 6  0.5,  0.5,  0.5,  1.0, 1.0,
                (glm::vec3(-0.5,  0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 1.0)), // 7
           // 4 -0.5, -0.5,  0.5,  0.0, 0.0,
            
                (glm::vec3(-0.5,  0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 0.0)), // 8
                (glm::vec3(-0.5,  0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)), // 9
                (glm::vec3(-0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 1.0)), // 10
          // 10 -0.5, -0.5, -0.5,  0.0, 1.0,
           // 4 -0.5, -0.5,  0.5,  0.0, 0.0,
           // 8 -0.5,  0.5,  0.5,  1.0, 0.0,
            
                (glm::vec3( 0.5,  0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 0.0)), // 11
           // 2  0.5,  0.5, -0.5,  1.0, 1.0,
                (glm::vec3( 0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 1.0)), // 12
          // 12  0.5, -0.5, -0.5,  0.0, 1.0,
                (glm::vec3( 0.5, -0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 0.0)), // 13
          // 11  0.5,  0.5,  0.5,  1.0, 0.0,
            
          // 10 -0.5, -0.5, -0.5,  0.0, 1.0,
                (glm::vec3( 0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)), // 14
           // 5  0.5, -0.5,  0.5,  1.0, 0.0,
           // 5  0.5, -0.5,  0.5,  1.0, 0.0,
           // 4 -0.5, -0.5,  0.5,  0.0, 0.0,
          // 10 -0.5, -0.5, -0.5,  0.0, 1.0,
            
           // 3 -0.5,  0.5, -0.5,  0.0, 1.0,
           // 2  0.5,  0.5, -0.5,  1.0, 1.0,
          // 11  0.5,  0.5,  0.5,  1.0, 0.0,
          // 11  0.5,  0.5,  0.5,  1.0, 0.0,
                (glm::vec3(-0.5,  0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 0.0)), // 15
           // 3 -0.5,  0.5, -0.5,  0.0, 1.0
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

        let (w, h) = self.window.get_framebuffer_size();
        
        let mut model = glm::identity::<f32, 4>();
        model = glm::rotate(&model, nalgebra_glm::radians(&glm::vec1(-55.0)).x, &glm::vec3(1.0, 0.0, 0.0));

        let mut view = glm::identity::<f32, 4>();
        view = glm::translate(&view, &glm::vec3(0.0, 0.0, -3.0));

        let projection = glm::perspective(w as f32 / h as f32, glm::radians(&glm::vec1(45.0)).x, 0.1, 100.0);

        println!("{}", projection * view * model);

        while !self.window.should_close() {
            self.gl.context.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                // This is only single until we get the rest of input working
                #[allow(clippy::single_match)]
                match event {
                    WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true);
                    },
                    WindowEvent::Key(Key::S, _, Action::Release, Modifiers::Shift) => {
                        view = glm::translate(&view, &glm::vec3(0.0, 0.0, -10.0));
                    },
                    WindowEvent::Key(Key::S, _, Action::Release, _) => {
                        view = glm::translate(&view, &glm::vec3(0.0, 0.0, -1.0));
                    },
                    WindowEvent::Key(Key::W, _, Action::Release, Modifiers::Shift) => {
                        view = glm::translate(&view, &glm::vec3(0.0, 0.0, 10.0));
                    },
                    WindowEvent::Key(Key::W, _, Action::Release, _) => {
                        view = glm::translate(&view, &glm::vec3(0.0, 0.0, 1.0));
                    },
                    WindowEvent::Key(Key::D, _, Action::Release, Modifiers::Shift) => {
                        view = glm::translate(&view, &glm::vec3(-10.0, 0.0, 0.0));
                    },
                    WindowEvent::Key(Key::D, _, Action::Release, _) => {
                        view = glm::translate(&view, &glm::vec3(-1.0, 0.0, 0.0));
                    },
                    WindowEvent::Key(Key::A, _, Action::Release, Modifiers::Shift) => {
                        view = glm::translate(&view, &glm::vec3(10.0, 0.0, 0.0));
                    },
                    WindowEvent::Key(Key::A, _, Action::Release, _) => {
                        view = glm::translate(&view, &glm::vec3(1.0, 0.0, 0.0));
                    },
                    _ => (),
                }
            }

            self.gl.clear_color(0.2, 0.3, 0.3, 1.0);
            self.gl.activate_shader(&shader);

            self.gl.specify_matrix_parameter(&shader, "model", &model);
            self.gl.specify_matrix_parameter(&shader, "view", &view);
            self.gl.specify_matrix_parameter(&shader, "projection", &projection);

            self.gl.activate_texture(&texture);
            self.gl.render_mesh(&cube_mesh);

            self.window.swap_buffers();
        }
    }
}
