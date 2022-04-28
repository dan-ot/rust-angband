use std::collections::HashMap;
use rusttype::{Font, Scale, point};
use image::RgbImage;

use crate::random::Random;
use crate::engine::texture::Texture;

/// This is actually a Tileset, not a font. If we were to go to 2D sprites or 3D models, this would all go away
/// but font-writing would not.
pub struct Cp437 {
    set: Vec<char>,
    code_map: HashMap<char, usize>,
    textures: Vec<Texture>
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
        let mut widest = 0;
        let rt_scale = Scale { x: scale, y: scale };

        for (i, c) in set.iter().enumerate() {
            map.insert(*c, i);
            let glyph = face.glyph(*c)
                .scaled(rt_scale)
                .positioned(point(0.0, 0.0));
            vec.push(Texture::from_glyph(&glyph, face.v_metrics(rt_scale)));
            widest = std::cmp::max(widest, glyph.pixel_bounding_box().map_or(0, |r| {r.width()}));
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
            let (converted_char, is_replacement) = Cp437::from_1242(i);

            vec.push(Texture::from_padded_image(&cp1252[converted_char], if is_replacement { normal_pad } else {normal_pad * 2}, scale));
        }

        Cp437 {
            set,
            code_map: map,
            textures: vec
        }
    }

    pub fn texture_from_code(&self, code: usize) -> &Texture {
        &self.textures[code]
    }

    pub fn char(&self, ch: char) -> &Texture {
        &self.textures[self.code_map[&ch]]
    }

    pub fn random(&self, rng: &mut Random) -> char {
        let pick = rng.randint0(self.set.len() as i32);
        self.set[pick as usize]
    }

    pub fn from_1242(code: usize) -> (usize, bool) {
        match code {
            0x00 | 0x20..=0x7E => (code, false),
            0x14 => (0xB6, false),
            0x15 => (0xA7, false),
            0x80 => (0xC7, false),
            0x81 => (0xFC, false),
            0x82 => (0xE9, false),
            0x83 => (0xE2, false),
            0x84 => (0xE4, false),
            0x85 => (0xE0, false),
            0x86 => (0xE5, false),
            0x88 => (0xEA, false),
            0x89 => (0xEB, false),
            0x8A => (0xE8, false),
            0x8B => (0xEF, false),
            0x8C => (0xEE, false),
            0x8D => (0xEC, false),
            0x8E => (0xC4, false),
            0x8F => (0xC5, false),
            0x90 => (0xC9, false),
            0x91 => (0xE6, false),
            0x92 => (0xC6, false),
            0x93 => (0xF4, false),
            0x94 => (0xF6, false),
            0x95 => (0xF3, false),
            0x96 => (0xFB, false),
            0x97 => (0xF9, false),
            0x98 => (0xFF, false),
            0x99 => (0xD6, false),
            0x9A => (0xDC, false),
            0x9B => (0xA2, false),
            0x9C => (0xA3, false),
            0x9D => (0xA5, false),
            0x9F => (0x83, false),
            0xA0 => (0xE1, false),
            0xA1 => (0xED, false),
            0xA2 => (0xF3, false),
            0xA3 => (0xFA, false),
            0xA4 => (0xF1, false),
            0xA5 => (0xD1, false),
            0xA9 => (0xBF, false),
            0xAA => (0xAC, false),
            0xAB => (0xBD, false),
            0xAC => (0xBC, false),
            0xAD => (0xA1, false),
            0xAE => (0xAB, false),
            0xAF => (0xBB, false),
            0xE1 => (0xDF, false),
            0xE6 => (0xB5, false),
            0xF1 => (0xB1, false),
            0xF6 => (0xF7, false),
            0xF8 => (0xB0, false),
            0xF9 => (0x95, false),
            0x07 | 0xFA => (0xB7, false),
            0xFD => (0xB2, false),
            _ => (0x3F, true)
        }
    }
}