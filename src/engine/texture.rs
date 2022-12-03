use rusttype::VMetrics;
use rusttype::PositionedGlyph;
use image::{ ColorType, RgbImage, Rgb };
use std::path::Path;
use std::convert::TryInto;
use std::ffi::c_void;
use image;

use crate::glad_gl::gl;

pub struct Texture {
    pub handle: u32,
    pub size: (u32, u32)
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
    pub fn from_file(source: &Path) -> Self {
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
            handle: tex, size: (pic.width(), pic.height())
        }
    }

    pub fn from_glyph(glyph: &PositionedGlyph, vmetrics: VMetrics) -> Self {
        let scale = glyph.scale();
        let mut target = RgbImage::new(scale.x as u32 + 1, scale.y as u32 + 1);
        let baseline = scale.y + vmetrics.descent;

        match glyph.pixel_bounding_box() {
            Some (bb) => {
                let left = (scale.x as i32 - bb.width()) / 2;
                let top = std::cmp::max(bb.min.y + (baseline as i32), 0);
                glyph.draw(|px, py, c| {
                    let color = (c * (u8::MAX as f32)) as u8;
                    let x = left as u32 + px;
                    let y = top as u32 + py;
                    if x < target.width() && y < target.height() && y > 0 {
                        target.put_pixel(x, y, Rgb([color, color, color]));
                    }
                });
            },
            None => {
            }
        }

        Texture::from_image(&target)
    }

    pub fn from_padded_image(img: &RgbImage, pad_px: u32, target_size: u32) -> Self {
        let mut image = RgbImage::new(target_size, target_size);

        let interior_dimension = target_size - (pad_px * 2);

        let scale_factor = interior_dimension / img.width();

        for i_x in 0..img.width() {
            for i_y in 0..img.height() {
                let [r, g, b] = img.get_pixel(i_x, i_y).0;

                for x in 0..scale_factor {
                    for y in 0..scale_factor {

                        image.put_pixel(
                            pad_px + (i_x * scale_factor) + x,
                            pad_px + (i_y * scale_factor) + y,
                            Rgb([r, g, b])
                        );
                    }
                }

            }
        }

        Texture::from_image(&image)
    }

    pub fn from_image(img: &RgbImage) -> Self {
        let mut tex = 0;

        unsafe {
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE.try_into().unwrap());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE.try_into().unwrap());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR.try_into().unwrap());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.try_into().unwrap());

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB8.try_into().unwrap(),
                img.width().try_into().unwrap(),
                img.height().try_into().unwrap(),
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                img.as_ptr() as *const c_void
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Texture {
            handle: tex, size: (img.width(), img.height())
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) { 
        unsafe {
            gl::DeleteTextures(1, [self.handle].as_ptr());
        }
     }
}