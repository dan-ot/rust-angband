use std::collections::HashMap;
use std::convert::TryInto;
use std::iter::zip;

use image::{Rgb, RgbImage};
use nalgebra_glm::{vec2, vec3, vec4, TVec2, TVec3, TVec4};
use rusttype::{point, Font, Scale};

use crate::engine::{texture::Texture, vertices::MeshKit};

/// This is a screen-renderable font - a single texture atlas and a mapping of character-to-tex-coords
/// to assist in rendering. The intent is for commonly-displayed characters (0x20 through 0xA5); the more fun corners of
/// CP437 are being left out.
pub struct Charmap {
    /// The OpenGL texture containing the rendered characters
    atlas: Texture,
    /// A map of displayable character to coordinate and font details
    map: HashMap<char, Character>,
}

pub struct Character {
    /// Position within the containing atlas - left, top, right, bottom
    texels: TVec4<f32>,
    /// The rusttype-provided bounding box (does not include position info...)
    bounds: TVec4<f32>,
    /// How much room to leave between characters
    advance: f32,
}

/// The origin of this line of text is left-justified on the baseline (not the bottom!)
pub struct Line<'a> {
    texture: &'a Texture,
    renderable: MeshKit,
}

impl Charmap {
    pub fn from_font(face: &Font, scale: f32) -> Self {
        let set = vec![
            // CP437 0x20
            '\u{0020}', '\u{0021}', '\u{0022}', '\u{0023}', '\u{0024}', '\u{0025}', '\u{0026}',
            '\u{0027}', '\u{0028}', '\u{0029}', '\u{002A}', '\u{002B}', '\u{002C}', '\u{002D}',
            '\u{002E}', '\u{002F}', '\u{0030}', '\u{0031}', '\u{0032}', '\u{0033}', '\u{0034}',
            '\u{0035}', '\u{0036}', '\u{0037}', '\u{0038}', '\u{0039}', '\u{003A}', '\u{003B}',
            '\u{003C}', '\u{003D}', '\u{003E}', '\u{003F}', '\u{0040}', '\u{0041}', '\u{0042}',
            '\u{0043}', '\u{0044}', '\u{0045}', '\u{0046}', '\u{0047}', '\u{0048}', '\u{0049}',
            '\u{004A}', '\u{004B}', '\u{004C}', '\u{004D}', '\u{004E}', '\u{004F}', '\u{0050}',
            '\u{0051}', '\u{0052}', '\u{0053}', '\u{0054}', '\u{0055}', '\u{0056}', '\u{0057}',
            '\u{0058}', '\u{0059}', '\u{005A}', '\u{005B}', '\u{005C}', '\u{005D}', '\u{005E}',
            '\u{005F}', '\u{0060}', '\u{0061}', '\u{0062}', '\u{0063}', '\u{0064}', '\u{0065}',
            '\u{0066}', '\u{0067}', '\u{0068}', '\u{0069}', '\u{006A}', '\u{006B}', '\u{006C}',
            '\u{006D}', '\u{006E}', '\u{006F}', '\u{0070}', '\u{0071}', '\u{0072}', '\u{0073}',
            '\u{0074}', '\u{0075}', '\u{0076}', '\u{0077}', '\u{0078}', '\u{0079}', '\u{007A}',
            '\u{007B}', '\u{007C}', '\u{007D}', '\u{007E}', '\u{2302}', '\u{00C7}', '\u{00FC}',
            '\u{00E9}', '\u{00E2}', '\u{00E4}', '\u{00E0}', '\u{00E5}', '\u{00E7}', '\u{00EA}',
            '\u{00EB}', '\u{00E8}', '\u{00EF}', '\u{00EE}', '\u{00EC}', '\u{00C4}', '\u{00C5}',
            '\u{00C9}', '\u{00E6}', '\u{00C6}', '\u{00F4}', '\u{00F6}', '\u{00F2}', '\u{00FB}',
            '\u{00F9}', '\u{00FF}', '\u{00D6}', '\u{00DC}', '\u{00A2}', '\u{00A3}', '\u{00A5}',
            '\u{20A7}', '\u{0192}', '\u{00E1}', '\u{00ED}', '\u{00F3}', '\u{00FA}', '\u{00F1}',
            '\u{00D1}', // CP437 0x5A
        ];

        let mut map = HashMap::with_capacity(set.len());
        let rt_scale = Scale { x: scale, y: scale };

        let max_width: u32 = (scale * 8.0).floor() as u32;

        // Extract all the sized and positioned glyphs we'll support
        let glyphs = set.into_iter().map(|c| {
            (
                c,
                face.glyph(c).scaled(rt_scale).positioned(point(0.0, 0.0)),
            )
        });

        // Calculate the relative rectangles in texels
        let mut l_off: u32 = 0;
        let mut t_off: u32 = 0;
        let in_texels: Vec<Option<TVec4<u32>>> = glyphs
            .clone()
            .map(|tup| {
                let (_, glyph) = tup;
                glyph.pixel_bounding_box().map(|bb| {
                    // Get this glyph's size
                    let glyph_width: u32 = bb.width().try_into().unwrap();
                    let glyph_height: u32 = bb.height().try_into().unwrap();

                    // Capture the bounds
                    let result = vec4(l_off, t_off, l_off + glyph_width, t_off + glyph_height);

                    // Mutability is obfuscating the result - but anyway...
                    // Check for row wrap
                    if l_off + glyph_width > max_width {
                        l_off = 0;
                        t_off += glyph_height;
                    } else {
                        l_off += glyph_width;
                    }

                    result
                })
            })
            .collect();

        let (tx, ty) = ((l_off + 1) as f32, (t_off + 1) as f32);

        // Because of mutability, l_off and t_off also include the width and height of the last character
        let mut image = RgbImage::new(l_off + 1, t_off + 1);

        for combo in zip(in_texels, glyphs) {
            let (possible_txls, (ch, gl)) = combo;
            let advance = gl.unpositioned().h_metrics().advance_width;
            let bounds = match gl.unpositioned().exact_bounding_box() {
                Some(ebb) => vec4(ebb.min.x, ebb.min.y, ebb.max.x, ebb.max.y),
                None => vec4(0.0, 0.0, 0.0, 0.0),
            };

            match possible_txls {
                Some(txl) => {
                    gl.draw(|px, py, a| {
                        let color = (a * (u8::MAX as f32)) as u8;
                        image.put_pixel(txl[0] + px, txl[1] + py, Rgb([color, color, color]));
                    });
                    let c = Character {
                        texels: vec4(
                            txl[0] as f32 / tx,
                            txl[1] as f32 / ty,
                            txl[2] as f32 / tx,
                            txl[3] as f32 / ty,
                        ),
                        bounds,
                        advance,
                    };
                    map.insert(ch, c);
                }
                None => {
                    let c = Character {
                        texels: vec4(0.0, 0.0, 1.0 / tx, 1.0 / ty),
                        bounds,
                        advance,
                    };
                    map.insert(ch, c);
                }
            };
        }
        Charmap {
            atlas: Texture::from_image(&image),
            map,
        }
    }

    pub fn line(&self, text: &str) -> Line {
        let data = text
            .chars()
            .into_iter()
            .enumerate()
            .scan(0_f32, |offset, (index, ch)| {
                let char_def = self.map.get(&ch).unwrap_or(self.map.get(&'?').unwrap());
                let coords = vec![
                    (
                        vec3(char_def.bounds.z + *offset, 0.0, char_def.bounds.y),
                        vec2(char_def.texels.z, char_def.texels.y),
                    ),
                    (
                        vec3(char_def.bounds.z + *offset, 0.0, char_def.bounds.w),
                        vec2(char_def.texels.z, char_def.texels.w),
                    ),
                    (
                        vec3(char_def.bounds.x + *offset, 0.0, char_def.bounds.w),
                        vec2(char_def.texels.x, char_def.texels.w),
                    ),
                    (
                        vec3(char_def.bounds.x + *offset, 0.0, char_def.bounds.y),
                        vec2(char_def.texels.x, char_def.texels.y),
                    ),
                ];
                *offset = char_def.bounds.z + char_def.advance;
                let my_indices = vec![
                    1 + (6 * index) as u32,
                    2 + (6 * index) as u32,
                    3 + (6 * index) as u32,
                    0 + (6 * index) as u32,
                    1 + (6 * index) as u32,
                    3 + (6 * index) as u32
                ];
                Some((coords, my_indices))
            });
            // .flatten();
        let vec = data
            .clone()
            .map(|tup| {tup.0})
            .flatten()
            .collect::<Vec<(TVec3<f32>, TVec2<f32>)>>();
            //.collect::<(Vec<(TVec3<f32>, TVec2<f32>)>, [usize; 6])>();
        let indices: Vec<u32> = data.map(|tup| {tup.1})
            .flatten()
            .collect();
        Line {
            texture: &self.atlas,
            renderable: MeshKit::new(&vec, &indices),
        }
    }
}
