use glfw::{Action, Context, Glfw, Key, Window, WindowEvent, WindowHint, WindowMode};
use std::convert::TryInto;
use std::mem::size_of;
use std::ffi::c_void;
use std::path::Path;
use std::sync::mpsc::Receiver;

// use crate::ui::FontAtlas;
use crate::glad_gl::gl;
use crate::ui::graphics::GraphicsModeService;

pub mod shader;
pub mod vertices;
pub mod glwrap;

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

        
        // gl::load(|e| context.get_proc_address_raw(e) as *const std::os::raw::c_void);
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
            (( 0.5, -0.5, 0.0), (1.0, 0.0, 1.0), (1.0, 0.0)),
            ((-0.5, -0.5, 0.0), (1.0, 1.0, 0.0), (0.0, 0.0)),
            ((-0.5,  0.5, 0.0), (1.0, 1.0, 0.0), (0.0, 1.0))
        ];
        let indices = [
            0, 1, 3,
            1, 2, 3
        ];
        let mesh = vertices::MeshKit::new(&vertex_data, &indices);
        // let mut vertex_array: u32 = 0;
        // let mut element_buffer: u32 = 0;
        // let mut vertex_buffer: u32 = 0;
        // let size: i32 = indices.len().try_into().unwrap();

        // let mut as_vec = Vec::<f32>::new();
        
        // for (vert, color, tex) in vertex_data.iter() {
        //     let (x, y, z) = vert;
        //     as_vec.extend([x, y, z]);
        //     let (r, g, b) = color;
        //     as_vec.extend([r, g, b]);
        //     let (tx, ty) = tex;
        //     as_vec.extend([tx, ty]);
        // }

        // let sf32 = size_of::<f32>();
        // unsafe {
        //     gl::GenVertexArrays(1, &mut vertex_array);
        //     gl::BindVertexArray(vertex_array);
            
        //     gl::GenBuffers(1, &mut vertex_buffer);
        //     gl::GenBuffers(1, &mut element_buffer);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        //     gl::BufferData(
        //         gl::ARRAY_BUFFER,
        //         (as_vec.len() * sf32).try_into().unwrap(),
        //         as_vec.as_ptr() as *const c_void,
        //         gl::STATIC_DRAW
        //     );
            
        //     gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer);
        //     gl::BufferData(
        //         gl::ELEMENT_ARRAY_BUFFER,
        //         (indices.len() * size_of::<u32>()).try_into().unwrap(),
        //         &indices as *const _ as *const c_void,
        //         gl::STATIC_DRAW
        //     );

        //     let info_per_vertex = 3 // for the vertex
        //         + 3 // for the color
        //         + 2; // for the texel
        //     let row_size: i32 = (info_per_vertex * sf32).try_into().unwrap();

        //     // In this case, we mean '0 cast to Any', not 'pointer to memory 0'
        //     #[allow(clippy::zero_ptr)]
        //     gl::VertexAttribPointer(
        //         0,
        //         3,
        //         gl::FLOAT,
        //         gl::FALSE,
        //         row_size,
        //         0 as *const c_void
        //     );
        //     gl::EnableVertexAttribArray(0);

        //     // Color binding
        //     gl::VertexAttribPointer(
        //         1,
        //         3,
        //         gl::FLOAT,
        //         gl::FALSE,
        //         row_size,
        //         // Offset by the size of the previous structure
        //         (3 * sf32) as *const c_void
        //     );
        //     gl::EnableVertexAttribArray(1);

        //     // Texture coord binding
        //     gl::VertexAttribPointer(
        //         2,
        //         2,
        //         gl::FLOAT,
        //         gl::FALSE,
        //         row_size,
        //         // Offset by the sizes of both previous structures
        //         ((3 + 3) * sf32) as *const c_void
        //     );
        //     gl::EnableVertexAttribArray(2);

        //     // Bind to 0 means release/unfocus
        //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // }
        // unsafe {
        //     let vertices: [f32; 24] = [
        //         // Positions     Colors
        //         0.5, 0.5, 0.0, 1.0, 1.0, 1.0, // Top Right
        //         0.5, -0.5, 0.0, 1.0, 0.0, 1.0, // Bottom Right
        //         -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, // Bottom Left
        //         -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, // Top Left
        //     ];
        //     let indices: [u32; 6] = [
        //         0, 1, 3, // First Triangle - TR->BR->TL, clockwise
        //         1, 2, 3, // Second Triangle - BR->BL->TL, clockwise
        //     ];
        //     gl::GenVertexArrays(1, &mut vao);
        //     gl::GenBuffers(1, &mut vbo);
        //     gl::GenBuffers(1, &mut ebo);
        //     gl::BindVertexArray(vao);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        //     gl::BufferData(
        //         gl::ARRAY_BUFFER,
        //         (vertices.len() * size_of::<f32>()).try_into().unwrap(),
        //         &vertices as *const _ as *const std::ffi::c_void,
        //         gl::STATIC_DRAW,
        //     );
        //     gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        //     gl::BufferData(
        //         gl::ELEMENT_ARRAY_BUFFER,
        //         (indices.len() * size_of::<u32>()).try_into().unwrap(),
        //         &indices as *const _ as *const std::ffi::c_void,
        //         gl::STATIC_DRAW,
        //     );
        //     gl::VertexAttribPointer(
        //         0,
        //         3,
        //         gl::FLOAT,
        //         gl::FALSE,
        //         (6 * size_of::<f32>()).try_into().unwrap(),
        //         0 as *const std::ffi::c_void,
        //     );

        //     gl::EnableVertexAttribArray(0);
        //     gl::VertexAttribPointer(
        //         1,
        //         3,
        //         gl::FLOAT,
        //         gl::FALSE,
        //         (6 * size_of::<f32>()).try_into().unwrap(),
        //         (3 * size_of::<f32>()) as *const std::ffi::c_void,
        //     );
        //     gl::EnableVertexAttribArray(1);

        //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // };

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

            // let time_value = self.context.get_time();
            // let green_value = ((time_value.sin() / 2.0) + 0.5) as f32;
            unsafe {
                // let vertex_color_location = gl::GetUniformLocation(shader_program, CStr::from_bytes_with_nul("ourColor\0".as_bytes()).unwrap().as_ptr());
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                shader.activate();
                mesh.render();

                // gl::BindVertexArray(vertex_array);
                // In this case, we mean '0 cast to Any', not 'pointer to memory 0'
                // #[allow(clippy::zero_ptr)]
                // gl::DrawElements(
                //     gl::TRIANGLES,
                //     size,
                //     gl::UNSIGNED_INT,
                //     0 as *const c_void
                // );
                // gl::BindVertexArray(0);
                // gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);
                // gl::BindVertexArray(vao);
                // gl::DrawElements(
                //     gl::TRIANGLES,
                //     6,
                //     gl::UNSIGNED_INT,
                //     0 as *const std::ffi::c_void,
                // );
                // gl::BindVertexArray(0);
            }

            self.window.swap_buffers();
        }

        // unsafe {
        //     gl::DeleteVertexArrays(1, &vertex_array);
        //     gl::DeleteBuffers(1, &vertex_buffer);
        //     gl::DeleteBuffers(1, &element_buffer);
        // }
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
