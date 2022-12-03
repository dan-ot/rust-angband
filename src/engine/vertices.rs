use nalgebra_glm::{vec2, vec3, Vec2, Vec3};
use std::convert::TryInto;
use std::ffi::c_void;
use std::mem::size_of;

use crate::glad_gl::gl;

pub struct MeshKit {
    pub vertex_control_handle: u32,
    pub vertex_data_handle: u32,
    pub element_handle: u32,
    pub size: i32,
    raw_data: Vec<f32>,
    indices: Vec<u32>,
}

impl MeshKit {
    /// Take the vertices, and zip them with each member of data, to make
    /// data-per-vertex. Will probably need to extend data storage for
    /// shaders that have inputs other than vec3
    pub fn new(vertices_with_data: &[(Vec3, Vec2)], raw_indices: &[u32]) -> MeshKit {
        let mut vao: u32 = 0;
        let mut ebo: u32 = 0;
        let mut vbo: u32 = 0;

        let mut as_vec = Vec::<f32>::new();
        let indices = Vec::from(raw_indices);

        for (vert, tex) in vertices_with_data.iter() {
            as_vec.extend([vert.x, vert.y, vert.z]);
            as_vec.extend([tex.x, tex.y]);
        }

        let sf32 = size_of::<f32>();

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (as_vec.len() * sf32).try_into().unwrap(),
                as_vec.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<u32>()).try_into().unwrap(),
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            let info_per_vertex = 3 // for the vertex
                + 2; // for the texel
            let row_size: i32 = (info_per_vertex * sf32).try_into().unwrap();

            // In this case, we mean '0 cast to Any', not 'pointer to memory 0'
            #[allow(clippy::zero_ptr)]
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, row_size, 0 as *const c_void);
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                row_size,
                (3 * sf32) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            // Bind to 0 means release/unfocus
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        MeshKit {
            vertex_control_handle: vao,
            vertex_data_handle: vbo,
            element_handle: ebo,
            size: indices.len().try_into().unwrap(),
            raw_data: as_vec,
            indices,
        }
    }

    pub fn quad_flat(left: f32, top: f32, right: f32, bottom: f32) -> MeshKit {
        MeshKit::new(
            &[
                (vec3(left, 0.0, top), vec2(1.0, 0.0)),
                (vec3(right, 0.0, top), vec2(0.0, 0.0)),
                (vec3(left, 0.0, bottom), vec2(1.0, 1.0)),
                (vec3(right, 0.0, bottom), vec2(0.0, 1.0)),
            ],
            &[0, 2, 3, 1, 0, 3],
        )
    }

    pub fn quad_standing(top_left: Vec2, bottom_right: Vec2) -> MeshKit {
        MeshKit::new(
            &[
                (vec3(bottom_right.x, top_left.y, 0.0), vec2(1.0, 1.0)),
                (vec3(bottom_right.x, bottom_right.y, 0.0), vec2(1.0, 0.0)),
                (vec3(top_left.x, bottom_right.y, 0.0), vec2(0.0, 0.0)),
                (vec3(top_left.x, top_left.y, 0.0), vec2(0.0, 1.0)),
            ],
            &[0, 1, 3, 1, 2, 3],
        )
    }

    pub fn boxy(x_span: Vec2, y_span: Vec2, z_span: Vec2) -> MeshKit {
        MeshKit::new(
            &[
                (vec3(x_span.x, y_span.x, z_span.x), vec2(0.0, 0.0)),
                (vec3(x_span.y, y_span.x, z_span.x), vec2(1.0, 0.0)),
                (vec3(x_span.y, y_span.y, z_span.x), vec2(1.0, 1.0)),
                (vec3(x_span.x,  y_span.y, z_span.x), vec2(0.0, 1.0)),
                (vec3(x_span.x, y_span.x, z_span.y), vec2(0.0, 0.0)),
                (vec3(x_span.y, y_span.x, z_span.y), vec2(1.0, 0.0)),
                (vec3(x_span.y, y_span.y, z_span.y), vec2(1.0, 1.0)),
                (vec3(x_span.x, y_span.y, z_span.y), vec2(0.0, 1.0)),
                (vec3(x_span.x, y_span.y, z_span.y), vec2(1.0, 0.0)),
                (vec3(x_span.x, y_span.y, z_span.x), vec2(1.0, 1.0)),
                (vec3(x_span.x, y_span.x, z_span.x), vec2(0.0, 1.0)),
                (vec3(x_span.y, y_span.y, z_span.y), vec2(1.0, 0.0)),
                (vec3(x_span.y, y_span.x, z_span.x), vec2(0.0, 1.0)),
                (vec3(x_span.y, y_span.x, z_span.y), vec2(0.0, 0.0)),
                (vec3(x_span.y, y_span.x, z_span.x), vec2(1.0, 1.0)),
                (vec3(x_span.x, y_span.y, z_span.y), vec2(0.0, 0.0))
            ],
            &[
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
            ]
        )
    }
}

impl Drop for MeshKit {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vertex_control_handle);
            gl::DeleteBuffers(
                2,
                &[self.vertex_data_handle, self.element_handle] as *const u32,
            );
        }
    }
}
