use image::{ DynamicImage, ColorType };
use std::path::Path;
use std::convert::TryInto;
use std::ffi::c_void;
use image;

use crate::glad_gl::gl;

pub struct Texture {
    pub handle: u32
}

fn interpret_color_type(source: ColorType) -> (i32, u32) {
    match (source.bytes_per_pixel(), source.channel_count()) {
        (3, 3) => (gl::RGB8.try_into().unwrap(), gl::UNSIGNED_BYTE),
        (4, 4) => (gl::RGBA8.try_into().unwrap(), gl::UNSIGNED_BYTE),
        (6, 3) => (gl::RGB16.try_into().unwrap(), gl::UNSIGNED_INT),
        (12, 4) => (gl::RGBA16.try_into().unwrap(), gl::UNSIGNED_INT),
        (12, 3) => (gl::RGB32F.try_into().unwrap(), gl::FLOAT),
        (16, 4) => (gl::RGBA32F.try_into().unwrap(), gl::FLOAT),
        _ => panic!("Unknown color detail!")
    }
}

impl Texture {
    pub fn new(source: &Path) -> Self {
        let pic = image::open(source).unwrap();
        let mut tex = 0;

        let (format, pixel_type) = interpret_color_type(pic.color());

        unsafe {
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.try_into().unwrap());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.try_into().unwrap());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR.try_into().unwrap());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.try_into().unwrap());

            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0, 
                format,
                pic.width() as i32,
                pic.height() as i32,
                0,
                gl::RGB,
                pixel_type,
                pic.as_bytes().as_ptr() as *const c_void
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Texture {
            handle: tex
        }
    }
}