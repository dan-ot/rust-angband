use glfw::{Action, Context, Glfw, Key, Window, WindowEvent, WindowHint, WindowMode};
use std::convert::TryInto;
use std::ffi::c_void;
use std::path::Path;
use std::sync::mpsc::Receiver;

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

        let vertex_data = [
            (( 0.5,  0.5, 0.0), (1.0, 1.0, 1.0), (1.0, 1.0)),
            (( 0.5, -0.5, 0.0), (1.0, 1.0, 1.0), (1.0, 0.0)),
            ((-0.5, -0.5, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0)),
            ((-0.5,  0.5, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0))
        ];
        let indices = [
            0, 1, 3,
            1, 2, 3
        ];
        let mesh = vertices::MeshKit::new(&vertex_data, &indices);

        let texture = texture::Texture::new(
            Path::new("resources/images/container.jpg")
        );

        while !self.window.should_close() {
            self.gl.context.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                // This is only single until we get the rest of input working
                #[allow(clippy::single_match)]
                match event {
                    WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true);
                    }
                    _ => (),
                }
            }

            self.gl.clear_color(0.2, 0.3, 0.3, 1.0);
            self.gl.activate_shader(&shader);
            self.gl.activate_texture(&texture);
            self.gl.render_mesh(&mesh);

            self.window.swap_buffers();
        }
    }
}
