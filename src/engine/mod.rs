use glfw::{Action, Context, Glfw, Key, Window, WindowEvent, WindowHint, WindowMode};
use std::convert::TryInto;
use std::ffi::CStr;
use std::mem::size_of;
use std::sync::mpsc::Receiver;

// use crate::ui::FontAtlas;
use crate::glad_gl::gl;
use crate::ui::graphics::GraphicsModeService;

pub struct Engine {
    pub context: Glfw,
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

        gl::load(|e| context.get_proc_address_raw(e) as *const std::os::raw::c_void);
        // let atlas = Box::new(FontAtlas::render_atlas(&graphics.fonts[graphics.current_font], &font_context, &mut canvas, &texture_creator));

        Engine {
            context,
            window,
            events,
            graphics_modes: graphics,
        }
    }

    pub fn run(&mut self) {
        let mut shader_program: u32;
        unsafe {
            let vertex_file_bytes =
                std::fs::read_to_string("resources/shaders/vertex_default.glsl").unwrap()
                + "\0";
            let vertex_shader_content =
                CStr::from_bytes_with_nul(vertex_file_bytes.as_bytes()).unwrap();
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &vertex_shader_content.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(vertex_shader);
            let mut vertex_success: i32 = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut vertex_success);
            if vertex_success == 0 {
                panic!("Vertex Shader Compilation failed!");
            }

            let fragment_file_bytes =
                std::fs::read_to_string("resources/shaders/fragment_default.glsl").unwrap()
                + "\0";
            let fragment_shader_content =
                CStr::from_bytes_with_nul(fragment_file_bytes.as_bytes()).unwrap();
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &fragment_shader_content.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(fragment_shader);

            let mut fragment_success: i32 = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut fragment_success);

            if fragment_success == 0 {
                panic!("Fragment Shader Compilation failed!");
            }
            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut program_success: i32 = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut program_success);
            if program_success == 0 {
                panic!("Shader Program Linking failed!");
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        let mut vbo: u32 = 0;
        let mut vao: u32 = 0;
        unsafe {
            let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<f32>()).try_into().unwrap(),
                &vertices as *const _ as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * size_of::<f32>()).try_into().unwrap(),
                0 as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        };

        while !self.window.should_close() {
            self.context.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true);
                    }
                    _ => (),
                }
            }

            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::UseProgram(shader_program);
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }

            self.window.swap_buffers();
        }

        unsafe {
            gl::DeleteVertexArrays(1, &vao);
            gl::DeleteBuffers(1, &vbo);
            gl::DeleteProgram(shader_program);
        }
        // 'running: loop {
        //     self.canvas.set_draw_color(Color::BLACK);
        //     self.canvas.clear();

        //     // TODO: DRAW
        //     // We'll need two things for this: a Graphics kit and a Font kit
        //     // To render a screen, we'll need a buffer (term?) of graphics or font references
        //     //  Maybe more than one buffer, for having a menu or 'subscreen' overlaid on the
        //     //  main menu or play field
        //     // We'll use these buffers to power our blits from the Graphics or Font kits

        //     self.canvas.present();
        //     for event in self.events.poll_iter() {
        //         match event {
        //             Event::Quit { .. }
        //             | Event::KeyDown {
        //                 keycode: Some(Keycode::Escape),
        //                 ..
        //             } => {
        //                 break 'running;
        //             }
        //             _ => {}
        //         }
        //     }
        // }
    }
}
