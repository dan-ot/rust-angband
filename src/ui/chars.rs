use std::collections::HashMap;
use std::convert::TryInto;

use image::{Rgb, RgbImage};
use nalgebra_glm::{vec2, vec3, vec4, TVec2, TVec3, TVec4};
use rusttype::{point, Font, Scale, Rect};

use crate::engine::{texture::Texture, vertices::MeshKit};

/// This is a screen-renderable font - a single texture atlas and a mapping of character-to-tex-coords
/// to assist in rendering. The intent is for commonly-displayed characters (0x20 through 0xA5); the more fun corners of
/// CP437 are being left out.
pub struct Charmap {
    /// The OpenGL texture containing the rendered characters
    pub atlas: Texture,
    /// A map of displayable character to coordinate and font details
    map: HashMap<char, Character>,
}

#[derive(Debug)]
pub struct Character {
    /// Position within the containing atlas - left, top, right, bottom
    texels: TVec4<f32>,
    /// The rusttype-provided bounding box (does not include position info...)
    quad: TVec4<f32>,
    /// How much room to leave between characters. For layout, add this to the next character in line
    /// and add 1 to the next line down
    advance: f32,
}

/// The origin of this line of text is left-justified on the baseline (not the bottom!)
pub struct Line<'a> {
    pub texture: &'a Texture,
    pub renderable: MeshKit,
}

/// Express the character coordinates into 3D space...
/// 
/// 1. Put the pixels without character spacing into a texture.
/// 2. Create a quad shortened from a 1x1 quad according to these edges.
///     * If you position the quad at (0,0) the character will be spaced relative to (1,advance)
/// 3. Map the texels of the quad to the texture-relative pixels rendered
///    without spacing. This leaves spacing (character and line) to the size of the quads
///    and their relative position in 3D space.
/// 4. Lay them out correctly by jumping one advance over for the next character, 
///    and one unit down for the next line
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderedDimensions {
    /// How many horizontal texels this will take up, for sprite packing
    pub rendered_width: u32,

    /// What fraction from a unit character origin the left edge is, for quad building
    pub left_edge: f32,
    /// What fraction from a unit character origin the right edge is, for quad building
    pub right_edge: f32,
    /// 3D space width of this character (for a monospace font, this will be the same always)
    /// relative to a 1x1 quad. In other words, the ratio of width to height for this character.
    pub advance: f32,

    /// How many vertical texels this will take up, for sprite packing
    pub rendered_height: u32,

    /// What fraction from a unit character origin the top is, for quad building.
    pub top_edge: f32,
    /// What fraction from a unit character origin the bottom is, for quad building.
    pub bottom_edge: f32
}

impl Charmap {
    pub fn from_font(face: &Font, scale: f32) -> Self {
        // OK, here's what we're trying to do...

        // Make room on a single texture for all the characters/symbols we need. In addition to getting
        // all the pixels together, we need to mark the boundaries of each symbol so we can refer to just
        // that one symbol. This includes a bit of other metadata used by the various rendering and layout
        // systems (the Character type)

        // SO first, we gather together the interesting symbols - this is Code Page 437 from IBM in the
        // early years, used by lots of ASCII games. These numbers are the Unicode reperesentation of
        // the code page.
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

        // This will be half our useful output - a map that lets us look up an input Unicode and refer
        // to the metadata (including how to find the pixels to display the character)

        // This is mutable here. Can we turn this into a list of things, that eventually becomes
        // a list of key-value pairs, which can be collapsed into this map?
        let mut map = HashMap::with_capacity(set.len());

        // This is a demand of the type rendering system - it has to be scaled to be rendered
        let rt_scale = Scale { x: scale, y: scale };

        // We decide to aim for 8 times the scale in width. A bit arbitrary.
        let max_width = (scale * 8.0).floor() as u32;

        // Translate each desired character into a pair with that chacter and the renderable
        // glyph
        let glyphs = set.into_iter().map(|c| {
            (
                c,
                // TODO: is positioning even important here?
                face.glyph(c).scaled(rt_scale),
            )
        });

        let height_portion_descent = face.v_metrics(rt_scale).descent;

        let bounds = glyphs.clone()
            .map(|tuple| {
                let (c, glyph) = tuple;
                let dimensions = glyph.exact_bounding_box()
                    .map(|bb| {
                        // These measurements describe the relationship of the final texture
                        // to the unit measurement for the font...this measurement only includes the rendered
                        // pixels - empty space to either side is left empty. In other words, when we go do
                        // the draw() call later, it's going to loop from bb.left to bb.right - not from 0 to glyph.advance
                        // Thus, our task is this - pack the pixels as densely as possible (left = 0 for 
                        // every render, no space for advance) but recreate the space in the renderable rects
                        // afterward. This means each rect will map to a different size in texels *and* be a
                        // different size in local space coords. Laying them out in unit space (1 x 1, then scaled)
                        // should then restore the natrual relationship to character spacing and baselines...
                        
                        // Origin is left-side baseline
                        // font.ascent + font.descent = rt_scale, so (ascent / -top) is how far above
                        // the baseline this character goes, and (-descent / bottom) is how far below
                        // the baseline this character goes. (advance / right) is how much width this
                        // character has.

                        let rendered_width = bb.width().round() as u32;
                        let left_edge = bb.min.x / rt_scale.x;
                        let right = bb.min.x + bb.width();
                        let right_edge = right / rt_scale.x;

                        let rendered_height = bb.height().round() as u32;
                        // Top is usually negative - distance from baseline to the topmost pixel
                        // Distance from quad origin (bottom) = 
                        // let top = height_portion_descent - bb.min.y;
                        // let top_edge = top / rt_scale.y;
                        // Bottom is sometimes negative (" floats far above the baseline)
                        let bottom = -height_portion_descent - bb.max.y;
                        let top = bottom + bb.height();
                        let bottom_edge = bottom / rt_scale.y;
                        let top_edge = top / rt_scale.y;

                        let advance = glyph.h_metrics().advance_width / rt_scale.x;
                        
                        RenderedDimensions {
                            rendered_width,
                            left_edge,
                            right_edge,
                            advance,

                            rendered_height,
                            top_edge,
                            bottom_edge
                        }
                    })
                    .unwrap_or(RenderedDimensions {
                        rendered_width: 0,
                        left_edge: 0.0,
                        right_edge: 0.0,
                        advance: glyph.h_metrics().advance_width / rt_scale.x,

                        rendered_height: 0,
                        top_edge: 0.0,
                        bottom_edge: 0.0
                    });
                (c, glyph, dimensions)
            });

        let mut rows = vec![];
        let _ = bounds
            .fold((&mut vec![], 0_u32),
                |so_far, item| {
                    let (this_row, left_offset) = so_far;
                    let (c, glyph, dimensions) = item;
                    let char_offset = if dimensions.rendered_width == 0 {
                        // If we didn't draw anything, no need to offset that last pixel
                        0
                    } else {
                        // If we drew something, need to move a pixel over so we don't overlap
                        dimensions.rendered_width
                    };
                    let potential_new_offset = left_offset + char_offset;
                    
                    if potential_new_offset > max_width {
                        // println!("Line break at [{}] - got to {} width", c, left_offset);
                        rows.push(this_row.clone());
                        this_row.clear();
                        this_row.push((c, glyph, dimensions, point(0_u32, 0_u32)));
                        (this_row, char_offset + 1)
                    } else {
                        this_row.push((c, glyph, dimensions, point(left_offset, 0_u32)));
                        (this_row, potential_new_offset + 1)
                    }
                } 
            );

        let largest_total_width = rows
            .iter()
            .map(|r| {r.last().map(|t| {t.3.x + t.2.rendered_width}).unwrap_or(0)})
            .max()
            .unwrap_or(0)
            + 1;

        let mut positioned_rows = vec!();
        let total_height = rows
            .iter()
            .fold(0_u32,
                |offset, row| {
                    let tallest = row.iter()
                        .map(|tup| {
                            let (_, _, dimensions, _) = tup;
                            dimensions.rendered_height
                        })
                        .max()
                        .unwrap_or(0);
                    positioned_rows.push(
                        row.iter()
                            .map(|tup| {
                                let (c, glyph, dimensions, position) = tup;
                                (*c, glyph, *dimensions, point(position.x, position.y + offset))
                            })
                            .collect::<Vec<_>>()
                    );
                    offset + tallest
                }
            )
            + 1;

        let mut rendered_packing = RgbImage::new(largest_total_width, total_height);

        positioned_rows.iter()
            .for_each(|row| {
                row.iter().for_each(|tup| {
                    let (c, glyph, dimensions, position) = *tup;
                    let positioned_glyph = glyph.clone().positioned(point(0.0, 0.0));
                    let bounds = positioned_glyph.pixel_bounding_box().unwrap_or(Rect::default());
                    positioned_glyph.draw(
                        |px, py, o| {
                            let color = (o * (u8::MAX as f32)) as u8;
                            // The actual pixel is the current pixel shifted to the left (to account
                            // for the padding the bounding box already has, which we don't want) and
                            // then positioned within the overall image
        
                            // The doc for .draw() states they're already unpadded - iterating from 0
                            // to width, not min.x to max.x
                            let x = px + position.x;
                            let y = py + position.y;
                            rendered_packing.put_pixel(x, y, Rgb([color, color, color]));
                        }
                    );
        
                    // The texel coordinates normalize the image's pixels into a 0.0 to 1.0 space
                    let texel = vec4(
                        (position.x as f32) / (largest_total_width as f32),
                        (position.y as f32) / (total_height as f32),
                        (position.x + bounds.width() as u32) as f32 / (largest_total_width as f32),
                        (position.y + bounds.height() as u32) as f32 / (total_height as f32)
                    );
        
                    let result = Character {
                        texels: texel,
                        quad: vec4(
                            dimensions.left_edge,
                            dimensions.top_edge,
                            dimensions.right_edge,
                            dimensions.bottom_edge
                        ),
                        advance: dimensions.advance
                    };

                    map.insert(
                        c,
                        result
                    );
                });
            });
        Charmap {
            atlas: Texture::from_image(&rendered_packing),
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
                // We swap our y coords here - OpenGL stores textures with 0 = bottom, and our calculations
                // have been 0 = top
                let coords = vec![
                    (
                        vec3(char_def.quad.x + *offset, 0.0, 1.0 - char_def.quad.y),
                        vec2(char_def.texels.x, char_def.texels.y),
                    ),
                    (
                        vec3(char_def.quad.z + *offset, 0.0, 1.0 - char_def.quad.y),
                        vec2(char_def.texels.z, char_def.texels.y),
                    ),
                    (
                        vec3(char_def.quad.x + *offset, 0.0, 1.0 - char_def.quad.w),
                        vec2(char_def.texels.x, char_def.texels.w),
                    ),
                    (
                        vec3(char_def.quad.z + *offset, 0.0, 1.0 - char_def.quad.w),
                        vec2(char_def.texels.z, char_def.texels.w),
                    ),
                ];
                *offset = *offset + char_def.advance;
                let my_indices = vec![
                    0 + (4 * index) as u32,
                    2 + (4 * index) as u32,
                    3 + (4 * index) as u32,
                    0 + (4 * index) as u32,
                    3 + (4 * index) as u32,
                    1 + (4 * index) as u32
                ];
                Some((coords, my_indices))
            });
            // .flatten();
        let vec = data
            .clone()
            .map(|tup| {tup.0})
            .flatten()
            .collect::<Vec<_>>();
            //.collect::<(Vec<(TVec3<f32>, TVec2<f32>)>, [usize; 6])>();
        let indices = data
            .map(|tup| {tup.1})
            .flatten()
            .collect::<Vec<_>>();
        Line {
            texture: &self.atlas,
            renderable: MeshKit::new(&vec, &indices),
        }
    }
}
