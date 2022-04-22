use std::collections::HashMap;
use rusttype::{Font, Scale, point};
use image::RgbImage;

use crate::random::Random;
use crate::engine::texture::Texture;

pub struct Cp437 {
    set: Vec<char>,
    code_map: HashMap<char, usize>,
    textures: Vec<Texture>,
}

impl Cp437 {
    pub fn from_face(face: &Font, scale: f32) -> Self {
        let set = vec!('\u{0020}', '\u{263A}', '\u{263B}', '\u{2665}', '\u{2666}', '\u{2663}', '\u{2660}', 
            '\u{2022}', '\u{25D8}', '\u{25CB}', '\u{25D9}', '\u{2642}', '\u{2640}', '\u{266A}', '\u{266B}', '\u{263C}', '\u{25BA}',
            '\u{25C4}', '\u{2195}', '\u{203C}', '\u{00B6}', '\u{00A7}', '\u{25AC}', '\u{21A8}', '\u{2191}', '\u{2193}', '\u{2192}',
            '\u{2190}', '\u{221F}', '\u{2194}', '\u{25B2}', '\u{25BC}', '\u{0020}', '\u{0021}', '\u{0022}', '\u{0023}', '\u{0024}',
            '\u{0025}', '\u{0026}', '\u{0027}', '\u{0028}', '\u{0029}', '\u{002A}', '\u{002B}', '\u{002C}', '\u{002D}', '\u{002E}',
            '\u{002F}', '\u{0030}', '\u{0031}', '\u{0032}', '\u{0033}', '\u{0034}', '\u{0035}', '\u{0036}', '\u{0037}', '\u{0038}',
            '\u{0039}', '\u{003A}', '\u{003B}', '\u{003C}', '\u{003D}', '\u{003E}', '\u{003F}', '\u{0040}', '\u{0041}', '\u{0042}',
            '\u{0043}', '\u{0044}', '\u{0045}', '\u{0046}', '\u{0047}', '\u{0048}', '\u{0049}', '\u{004A}', '\u{004B}', '\u{004C}',
            '\u{004D}', '\u{004E}', '\u{004F}', '\u{0050}', '\u{0051}', '\u{0052}', '\u{0053}', '\u{0054}', '\u{0055}', '\u{0056}', 
            '\u{0057}', '\u{0058}', '\u{0059}', '\u{005A}', '\u{005B}', '\u{005C}', '\u{005D}', '\u{005E}', '\u{005F}', '\u{0060}',
            '\u{0061}', '\u{0062}', '\u{0063}', '\u{0064}', '\u{0065}', '\u{0066}', '\u{0067}', '\u{0068}', '\u{0069}', '\u{006A}', 
            '\u{006B}', '\u{006C}', '\u{006D}', '\u{006E}', '\u{006F}', '\u{0070}', '\u{0071}', '\u{0072}', '\u{0073}', '\u{0074}',
            '\u{0075}', '\u{0076}', '\u{0077}', '\u{0078}', '\u{0079}', '\u{007A}', '\u{007B}', '\u{007C}', '\u{007D}', '\u{007E}',
            '\u{2302}', '\u{00C7}', '\u{00FC}', '\u{00E9}', '\u{00E2}', '\u{00E4}', '\u{00E0}', '\u{00E5}', '\u{00E7}', '\u{00EA}',
            '\u{00EB}', '\u{00E8}', '\u{00EF}', '\u{00EE}', '\u{00EC}', '\u{00C4}', '\u{00C5}', '\u{00C9}', '\u{00E6}', '\u{00C6}',
            '\u{00F4}', '\u{00F6}', '\u{00F2}', '\u{00FB}', '\u{00F9}', '\u{00FF}', '\u{00D6}', '\u{00DC}', '\u{00A2}', '\u{00A3}',
            '\u{00A5}', '\u{20A7}', '\u{0192}', '\u{00E1}', '\u{00ED}', '\u{00F3}', '\u{00FA}', '\u{00F1}', '\u{00D1}', '\u{00AA}',
            '\u{00BA}', '\u{00BF}', '\u{2310}', '\u{00AC}', '\u{00BD}', '\u{00BC}', '\u{00A1}', '\u{00AB}', '\u{00BB}', '\u{2591}',
            '\u{2592}', '\u{2593}', '\u{2502}', '\u{2524}', '\u{2561}', '\u{2562}', '\u{2556}', '\u{2555}', '\u{2563}', '\u{2551}',
            '\u{2557}', '\u{255D}', '\u{255C}', '\u{255B}', '\u{2510}', '\u{2514}', '\u{2534}', '\u{252C}', '\u{251C}', '\u{2500}',
            '\u{253C}', '\u{255E}', '\u{255F}', '\u{255A}', '\u{2554}', '\u{2569}', '\u{2566}', '\u{2560}', '\u{2550}', '\u{256C}',
            '\u{2567}', '\u{2568}', '\u{2564}', '\u{2565}', '\u{2559}', '\u{2558}', '\u{2552}', '\u{2553}', '\u{256B}', '\u{256A}',
            '\u{2518}', '\u{250C}', '\u{2588}', '\u{2584}', '\u{258C}', '\u{2590}', '\u{2580}', '\u{03B1}', '\u{00DF}', '\u{0393}',
            '\u{03C0}', '\u{03A3}', '\u{03C3}', '\u{00B5}', '\u{03C4}', '\u{03A6}', '\u{0398}', '\u{03A9}', '\u{03B4}', '\u{221E}',
            '\u{03C6}', '\u{03B5}', '\u{2229}', '\u{2261}', '\u{00B1}', '\u{2265}', '\u{2264}', '\u{2320}', '\u{2321}', '\u{00F7}',
            '\u{2248}', '\u{00B0}', '\u{2219}', '\u{00B7}', '\u{221A}', '\u{207F}', '\u{00B2}', '\u{25A0}', '\u{00A0}'
        );
        let mut map = HashMap::with_capacity(set.len());
        let mut vec = Vec::<Texture>::with_capacity(set.len());
        let rt_scale = Scale { x: scale, y: scale };

        for (i, c) in set.iter().enumerate() {
            map.insert(*c, i);
            let glyph = face.glyph(*c)
                .scaled(rt_scale)
                .positioned(point(0.0, 0.0));
            vec.push(Texture::from_glyph(&glyph, face.v_metrics(rt_scale)));
        }

        Cp437 {
            set,
            code_map: map,
            textures: vec
        }
    }

    pub fn from_cp1252(cp1252: Vec<RgbImage>, scale: u32) -> Self {
        let set = vec!('\u{0020}', '\u{263A}', '\u{263B}', '\u{2665}', '\u{2666}', '\u{2663}', '\u{2660}', 
            '\u{2022}', '\u{25D8}', '\u{25CB}', '\u{25D9}', '\u{2642}', '\u{2640}', '\u{266A}', '\u{266B}', '\u{263C}', '\u{25BA}',
            '\u{25C4}', '\u{2195}', '\u{203C}', '\u{00B6}', '\u{00A7}', '\u{25AC}', '\u{21A8}', '\u{2191}', '\u{2193}', '\u{2192}',
            '\u{2190}', '\u{221F}', '\u{2194}', '\u{25B2}', '\u{25BC}', '\u{0020}', '\u{0021}', '\u{0022}', '\u{0023}', '\u{0024}',
            '\u{0025}', '\u{0026}', '\u{0027}', '\u{0028}', '\u{0029}', '\u{002A}', '\u{002B}', '\u{002C}', '\u{002D}', '\u{002E}',
            '\u{002F}', '\u{0030}', '\u{0031}', '\u{0032}', '\u{0033}', '\u{0034}', '\u{0035}', '\u{0036}', '\u{0037}', '\u{0038}',
            '\u{0039}', '\u{003A}', '\u{003B}', '\u{003C}', '\u{003D}', '\u{003E}', '\u{003F}', '\u{0040}', '\u{0041}', '\u{0042}',
            '\u{0043}', '\u{0044}', '\u{0045}', '\u{0046}', '\u{0047}', '\u{0048}', '\u{0049}', '\u{004A}', '\u{004B}', '\u{004C}',
            '\u{004D}', '\u{004E}', '\u{004F}', '\u{0050}', '\u{0051}', '\u{0052}', '\u{0053}', '\u{0054}', '\u{0055}', '\u{0056}', 
            '\u{0057}', '\u{0058}', '\u{0059}', '\u{005A}', '\u{005B}', '\u{005C}', '\u{005D}', '\u{005E}', '\u{005F}', '\u{0060}',
            '\u{0061}', '\u{0062}', '\u{0063}', '\u{0064}', '\u{0065}', '\u{0066}', '\u{0067}', '\u{0068}', '\u{0069}', '\u{006A}', 
            '\u{006B}', '\u{006C}', '\u{006D}', '\u{006E}', '\u{006F}', '\u{0070}', '\u{0071}', '\u{0072}', '\u{0073}', '\u{0074}',
            '\u{0075}', '\u{0076}', '\u{0077}', '\u{0078}', '\u{0079}', '\u{007A}', '\u{007B}', '\u{007C}', '\u{007D}', '\u{007E}',
            '\u{2302}', '\u{00C7}', '\u{00FC}', '\u{00E9}', '\u{00E2}', '\u{00E4}', '\u{00E0}', '\u{00E5}', '\u{00E7}', '\u{00EA}',
            '\u{00EB}', '\u{00E8}', '\u{00EF}', '\u{00EE}', '\u{00EC}', '\u{00C4}', '\u{00C5}', '\u{00C9}', '\u{00E6}', '\u{00C6}',
            '\u{00F4}', '\u{00F6}', '\u{00F2}', '\u{00FB}', '\u{00F9}', '\u{00FF}', '\u{00D6}', '\u{00DC}', '\u{00A2}', '\u{00A3}',
            '\u{00A5}', '\u{20A7}', '\u{0192}', '\u{00E1}', '\u{00ED}', '\u{00F3}', '\u{00FA}', '\u{00F1}', '\u{00D1}', '\u{00AA}',
            '\u{00BA}', '\u{00BF}', '\u{2310}', '\u{00AC}', '\u{00BD}', '\u{00BC}', '\u{00A1}', '\u{00AB}', '\u{00BB}', '\u{2591}',
            '\u{2592}', '\u{2593}', '\u{2502}', '\u{2524}', '\u{2561}', '\u{2562}', '\u{2556}', '\u{2555}', '\u{2563}', '\u{2551}',
            '\u{2557}', '\u{255D}', '\u{255C}', '\u{255B}', '\u{2510}', '\u{2514}', '\u{2534}', '\u{252C}', '\u{251C}', '\u{2500}',
            '\u{253C}', '\u{255E}', '\u{255F}', '\u{255A}', '\u{2554}', '\u{2569}', '\u{2566}', '\u{2560}', '\u{2550}', '\u{256C}',
            '\u{2567}', '\u{2568}', '\u{2564}', '\u{2565}', '\u{2559}', '\u{2558}', '\u{2552}', '\u{2553}', '\u{256B}', '\u{256A}',
            '\u{2518}', '\u{250C}', '\u{2588}', '\u{2584}', '\u{258C}', '\u{2590}', '\u{2580}', '\u{03B1}', '\u{00DF}', '\u{0393}',
            '\u{03C0}', '\u{03A3}', '\u{03C3}', '\u{00B5}', '\u{03C4}', '\u{03A6}', '\u{0398}', '\u{03A9}', '\u{03B4}', '\u{221E}',
            '\u{03C6}', '\u{03B5}', '\u{2229}', '\u{2261}', '\u{00B1}', '\u{2265}', '\u{2264}', '\u{2320}', '\u{2321}', '\u{00F7}',
            '\u{2248}', '\u{00B0}', '\u{2219}', '\u{00B7}', '\u{221A}', '\u{207F}', '\u{00B2}', '\u{25A0}', '\u{00A0}'
        );

        let mut map = HashMap::with_capacity(set.len());
        let mut vec = Vec::<Texture>::with_capacity(set.len());

        let normal_pad = scale / 8;

        for (i, c) in set.iter().enumerate() {
            map.insert(*c, i);
            match i {
                0x00 | 0x20..=0x7E => {
                    vec.push(Texture::from_padded_image(&cp1252[i], normal_pad, scale))
                }
                0x14 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xB6], normal_pad, scale))
                }
                0x15 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xA7], normal_pad, scale))
                }
                0x80 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xC7], normal_pad, scale))
                }
                0x81 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xFC], normal_pad, scale))
                }
                0x82 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xE9], normal_pad, scale))
                }
                0x83 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xE2], normal_pad, scale))
                }
                0x84 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xE4], normal_pad, scale))
                }
                0x85 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xE0], normal_pad, scale))
                }
                0x86 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xE5], normal_pad, scale))
                }
                0x88 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xEA], normal_pad, scale))
                }
                0x89 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xEB], normal_pad, scale))
                }
                0x8A => {
                    vec.push(Texture::from_padded_image(&cp1252[0xE8], normal_pad, scale))
                }
                0x8B => {
                    vec.push(Texture::from_padded_image(&cp1252[0xEF], normal_pad, scale))
                }
                0x8C => {
                    vec.push(Texture::from_padded_image(&cp1252[0xEE], normal_pad, scale))
                }
                0x8D => {
                    vec.push(Texture::from_padded_image(&cp1252[0xEC], normal_pad, scale))
                }
                0x8E => {
                    vec.push(Texture::from_padded_image(&cp1252[0xC4], normal_pad, scale))
                }
                0x8F => {
                    vec.push(Texture::from_padded_image(&cp1252[0xC5], normal_pad, scale))
                }
                0x90 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xC9], normal_pad, scale))
                }
                0x91 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xE6], normal_pad, scale))
                }
                0x92 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xC6], normal_pad, scale))
                }
                0x93 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xF4], normal_pad, scale))
                }
                0x94 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xF6], normal_pad, scale))
                }
                0x95 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xF3], normal_pad, scale))
                }
                0x96 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xFB], normal_pad, scale))
                }
                0x97 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xF9], normal_pad, scale))
                }
                0x98 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xFF], normal_pad, scale))
                }
                0x99 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xD6], normal_pad, scale))
                }
                0x9A => {
                    vec.push(Texture::from_padded_image(&cp1252[0xDC], normal_pad, scale))
                }
                0x9B => {
                    vec.push(Texture::from_padded_image(&cp1252[0xA2], normal_pad, scale))
                }
                0x9C => {
                    vec.push(Texture::from_padded_image(&cp1252[0xA3], normal_pad, scale))
                }
                0x9D => {
                    vec.push(Texture::from_padded_image(&cp1252[0xA5], normal_pad, scale))
                }
                0x9F => {
                    vec.push(Texture::from_padded_image(&cp1252[0x83], normal_pad, scale))
                }
                0xA0 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xE1], normal_pad, scale))
                }
                0xA1 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xED], normal_pad, scale))
                }
                0xA2 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xF3], normal_pad, scale))
                }
                0xA3 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xFA], normal_pad, scale))
                }
                0xA4 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xF1], normal_pad, scale))
                }
                0xA5 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xD1], normal_pad, scale))
                }
                0xA9 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xBF], normal_pad, scale))
                }
                0xAA => {
                    vec.push(Texture::from_padded_image(&cp1252[0xAC], normal_pad, scale))
                }
                0xAB => {
                    vec.push(Texture::from_padded_image(&cp1252[0xBD], normal_pad, scale))
                }
                0xAC => {
                    vec.push(Texture::from_padded_image(&cp1252[0xBC], normal_pad, scale))
                }
                0xAD => {
                    vec.push(Texture::from_padded_image(&cp1252[0xA1], normal_pad, scale))
                }
                0xAE => {
                    vec.push(Texture::from_padded_image(&cp1252[0xAB], normal_pad, scale))
                }
                0xAF => {
                    vec.push(Texture::from_padded_image(&cp1252[0xBB], normal_pad, scale))
                }
                0xE1 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xDF], normal_pad, scale))
                }
                0xE6 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xB5], normal_pad, scale))
                }
                0xF1 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xB1], normal_pad, scale))
                }
                0xF6 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xF7], normal_pad, scale))
                }
                0xF8 => {
                    vec.push(Texture::from_padded_image(&cp1252[0xB0], normal_pad, scale))
                }
                0xF9 => {
                    vec.push(Texture::from_padded_image(&cp1252[0x95], normal_pad, scale))
                }
                0x07 | 0xFA => {
                    vec.push(Texture::from_padded_image(&cp1252[0xB7], normal_pad, scale))
                }
                0xFD => {
                    vec.push(Texture::from_padded_image(&cp1252[0xB2], normal_pad, scale))
                }
                _ => {
                    vec.push(Texture::from_padded_image(&cp1252[0x3F], normal_pad * 2, scale))
                }
            }
        }

        Cp437 {
            set,
            code_map: map,
            textures: vec
        }
    }

    pub fn code(&self, code: usize) -> &Texture {
        &self.textures[code]
    }

    pub fn char(&self, ch: char) -> &Texture {
        &self.textures[self.code_map[&ch]]
    }

    pub fn random(&self, rng: &mut Random) -> char {
        let pick = rng.randint0(self.set.len() as i32);
        self.set[pick as usize]
    }
}