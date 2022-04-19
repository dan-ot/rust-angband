use crate::random::Random;
use std::collections::HashMap;
use std::iter::FromIterator;
use nalgebra_glm::{vec3, TVec3};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Colors {
    Dark = 0,
    White,
    Slate,
    Orange,
    Red,
    Green,
    Blue,
    Umber,
    LDark,
    LWhite,
    LPurple,
    Yellow,
    LRed,
    LGreen,
    LBlue,
    LUmber,
    Purple,
    Violet,
    Teal,
    Mud,
    LYellow,
    Magenta,
    LTeal,
    LViolet,
    LPink,
    Mustard,
    BlueSlate,
    DeepLBlue,
    Shade,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Attrs {
    Full,
    Mono,
    Vga,
    Blind,
    Light,
    Dark,
    High,
    Metal,
    Misc,
}

pub enum BackgroundStyle {
    Black = 0,
    Same,
    Dark,
    Max,
}

pub struct ColorType {
    pub name: String,
    pub color_translate: HashMap<Attrs, Colors>,
}

impl ColorType {
    pub fn new(
        name: &str,
        full: Colors,
        mono: Colors,
        vga: Colors,
        blind: Colors,
        lighter: Colors,
        darker: Colors,
        highlight: Colors,
        metallic: Colors,
        misc: Colors,
    ) -> ColorType {
        ColorType {
            name: String::from(name),
            color_translate: HashMap::from_iter([
                (Attrs::Full, full),
                (Attrs::Mono, mono),
                (Attrs::Vga, vga),
                (Attrs::Blind, blind),
                (Attrs::Light, lighter),
                (Attrs::Dark, darker),
                (Attrs::High, highlight),
                (Attrs::Metal, metallic),
                (Attrs::Misc, misc),
            ]),
        }
    }
}

pub struct ColorService {
    pub angband_color_table: HashMap<Colors, TVec3<f32>>,
    pub color_table: HashMap<char, ColorType>,
    pub gamma_table: [u8; 256],
    pub gamma_helper: [i64; 256],
}

impl ColorService {
    pub fn new() -> ColorService {
        let gamma_helper = [
            0, -1420, -1242, -1138, -1065, -1007, -961, -921, -887, -857, -830, -806, -783, -762,
            -744, -726, -710, -694, -679, -666, -652, -640, -628, -617, -606, -596, -586, -576,
            -567, -577, -549, -541, -532, -525, -517, -509, -502, -495, -488, -482, -475, -469,
            -463, -457, -451, -455, -439, -434, -429, -423, -418, -413, -408, -403, -398, -394,
            -389, -385, -380, -376, -371, -367, -363, -359, -355, -351, -347, -343, -339, -336,
            -332, -328, -325, -321, -318, -314, -311, -308, -304, -301, -298, -295, -291, -288,
            -285, -282, -279, -276, -273, -271, -268, -265, -262, -259, -257, -254, -251, -248,
            -246, -243, -241, -238, -236, -233, -231, -228, -226, -223, -221, -219, -216, -214,
            -212, -209, -207, -205, -203, -200, -198, -196, -194, -192, -190, -188, -186, -184,
            -182, -180, -178, -176, -174, -172, -170, -168, -166, -164, -162, -160, -158, -156,
            -155, -153, -151, -149, -147, -146, -144, -142, -140, -139, -137, -135, -134, -132,
            -130, -128, -127, -125, -124, -122, -120, -119, -117, -116, -114, -112, -111, -109,
            -108, -106, -105, -103, -102, -100, -99, -97, -96, -95, -93, -92, -90, -89, -87, -86,
            -85, -83, -82, -80, -79, -78, -76, -75, -74, -72, -71, -70, -68, -67, -66, -65, -63,
            -62, -61, -59, -58, -57, -56, -54, -53, -52, -51, -50, -48, -47, -46, -45, -44, -42,
            -41, -40, -39, -38, -37, -35, -34, -33, -32, -31, -30, -29, -27, -26, -25, -24, -23,
            -22, -21, -20, -19, -18, -17, -16, -14, -13, -12, -11, -10, -9, -8, -7, -6, -5, -4, -3,
            -2, -1,
        ];
        ColorService {
            angband_color_table: HashMap::from_iter([
                (Colors::Dark, vec3((0x00 as f32) / 256.0, (0x00 as f32) / 256.0, (0x00 as f32) / 256.0)),
                (Colors::White, vec3((0xff as f32) / 256.0, (0xff as f32) / 256.0, (0xff as f32) / 256.0)),
                (Colors::Slate, vec3((0x80 as f32) / 256.0, (0x80 as f32) / 256.0, (0x80 as f32) / 256.0)),
                (Colors::Orange, vec3((0xff as f32) / 256.0, (0x80 as f32) / 256.0, (0x00 as f32) / 256.0)),
                (Colors::Red, vec3((0xc0 as f32) / 256.0, (0x00 as f32) / 256.0, (0x00 as f32) / 256.0)),
                (Colors::Green, vec3((0x00 as f32) / 256.0, (0x80 as f32) / 256.0, (0x40 as f32) / 256.0)),
                (Colors::Blue, vec3((0x00 as f32) / 256.0, (0x40 as f32) / 256.0, (0xff as f32) / 256.0)),
                (Colors::Umber, vec3((0x80 as f32) / 256.0, (0x40 as f32) / 256.0, (0x00 as f32) / 256.0)),
                (Colors::LDark, vec3((0x60 as f32) / 256.0, (0x60 as f32) / 256.0, (0x60 as f32) / 256.0)),
                (Colors::LWhite, vec3((0xc0 as f32) / 256.0, (0xc0 as f32) / 256.0, (0xc0 as f32) / 256.0)),
                (Colors::LPurple, vec3((0xff as f32) / 256.0, (0x00 as f32) / 256.0, (0xff as f32) / 256.0)),
                (Colors::Yellow, vec3((0xff as f32) / 256.0, (0xff as f32) / 256.0, (0x00 as f32) / 256.0)),
                (Colors::LRed, vec3((0xff as f32) / 256.0, (0x40 as f32) / 256.0, (0x40 as f32) / 256.0)),
                (Colors::LGreen, vec3((0x00 as f32) / 256.0, (0xff as f32) / 256.0, (0x00 as f32) / 256.0)),
                (Colors::LBlue, vec3((0x00 as f32) / 256.0, (0xff as f32) / 256.0, (0xff as f32) / 256.0)),
                (Colors::LUmber, vec3((0xc0 as f32) / 256.0, (0x80 as f32) / 256.0, (0x40 as f32) / 256.0)),
                (Colors::Purple, vec3((0x90 as f32) / 256.0, (0x00 as f32) / 256.0, (0x90 as f32) / 256.0)),
                (Colors::Violet, vec3((0x90 as f32) / 256.0, (0x20 as f32) / 256.0, (0xff as f32) / 256.0)),
                (Colors::Teal, vec3((0x00 as f32) / 256.0, (0xa0 as f32) / 256.0, (0xa0 as f32) / 256.0)),
                (Colors::Mud, vec3((0x6c as f32) / 256.0, (0x6c as f32) / 256.0, (0x30 as f32) / 256.0)),
                (Colors::LYellow, vec3((0xff as f32) / 256.0, (0xff as f32) / 256.0, (0x90 as f32) / 256.0)),
                (Colors::Magenta, vec3((0xff as f32) / 256.0, (0x00 as f32) / 256.0, (0xa0 as f32) / 256.0)),
                (Colors::LTeal, vec3((0x20 as f32) / 256.0, (0xff as f32) / 256.0, (0xdc as f32) / 256.0)),
                (Colors::LViolet, vec3((0xb8 as f32) / 256.0, (0xa8 as f32) / 256.0, (0xff as f32) / 256.0)),
                (Colors::LPink, vec3((0xff as f32) / 256.0, (0x80 as f32) / 256.0, (0x80 as f32) / 256.0)),
                (Colors::Mustard, vec3((0xb4 as f32) / 256.0, (0xb4 as f32) / 256.0, (0x00 as f32) / 256.0)),
                (Colors::BlueSlate, vec3((0xa0 as f32) / 256.0, (0xc0 as f32) / 256.0, (0xd0 as f32) / 256.0)),
                (Colors::DeepLBlue, vec3((0x00 as f32) / 256.0, (0xb0 as f32) / 256.0, (0xff as f32) / 256.0)),
                (Colors::Shade, vec3((0x28 as f32) / 256.0, (0x28 as f32) / 256.0, (0x28 as f32) / 256.0)),
            ]),
            color_table: HashMap::from_iter([
                (
                    'd',
                    ColorType::new(
                        "Dark",
                        Colors::Dark,
                        Colors::Dark,
                        Colors::Dark,
                        Colors::Dark,
                        Colors::LDark,
                        Colors::Dark,
                        Colors::LDark,
                        Colors::LDark,
                        Colors::Dark,
                    ),
                ),
                (
                    'w',
                    ColorType::new(
                        "White",
                        Colors::White,
                        Colors::White,
                        Colors::White,
                        Colors::White,
                        Colors::Yellow,
                        Colors::LWhite,
                        Colors::LBlue,
                        Colors::Yellow,
                        Colors::White,
                    ),
                ),
                (
                    's',
                    ColorType::new(
                        "Slate",
                        Colors::Slate,
                        Colors::White,
                        Colors::Slate,
                        Colors::Slate,
                        Colors::White,
                        Colors::LDark,
                        Colors::LWhite,
                        Colors::LWhite,
                        Colors::Slate,
                    ),
                ),
                (
                    'o',
                    ColorType::new(
                        "Orange",
                        Colors::Orange,
                        Colors::White,
                        Colors::Orange,
                        Colors::LWhite,
                        Colors::Yellow,
                        Colors::Slate,
                        Colors::Yellow,
                        Colors::Yellow,
                        Colors::Orange,
                    ),
                ),
                (
                    'r',
                    ColorType::new(
                        "Red",
                        Colors::Red,
                        Colors::White,
                        Colors::Red,
                        Colors::Slate,
                        Colors::LRed,
                        Colors::Slate,
                        Colors::LRed,
                        Colors::LRed,
                        Colors::Red,
                    ),
                ),
                (
                    'g',
                    ColorType::new(
                        "Green",
                        Colors::Green,
                        Colors::White,
                        Colors::Green,
                        Colors::Slate,
                        Colors::LGreen,
                        Colors::Slate,
                        Colors::LGreen,
                        Colors::LGreen,
                        Colors::Green,
                    ),
                ),
                (
                    'b',
                    ColorType::new(
                        "Blue",
                        Colors::Blue,
                        Colors::White,
                        Colors::Blue,
                        Colors::Slate,
                        Colors::LBlue,
                        Colors::Slate,
                        Colors::LBlue,
                        Colors::LBlue,
                        Colors::Blue,
                    ),
                ),
                (
                    'u',
                    ColorType::new(
                        "Umber",
                        Colors::Umber,
                        Colors::White,
                        Colors::Umber,
                        Colors::LDark,
                        Colors::LUmber,
                        Colors::LDark,
                        Colors::LUmber,
                        Colors::LUmber,
                        Colors::Umber,
                    ),
                ),
                (
                    'D',
                    ColorType::new(
                        "Light Dark",
                        Colors::LDark,
                        Colors::White,
                        Colors::LDark,
                        Colors::LDark,
                        Colors::Slate,
                        Colors::LDark,
                        Colors::Slate,
                        Colors::Slate,
                        Colors::LDark,
                    ),
                ),
                (
                    'W',
                    ColorType::new(
                        "Light Slate",
                        Colors::LWhite,
                        Colors::White,
                        Colors::LWhite,
                        Colors::LWhite,
                        Colors::White,
                        Colors::Slate,
                        Colors::White,
                        Colors::White,
                        Colors::Slate,
                    ),
                ),
                (
                    'P',
                    ColorType::new(
                        "Light Purple",
                        Colors::LPurple,
                        Colors::White,
                        Colors::LPurple,
                        Colors::Slate,
                        Colors::Yellow,
                        Colors::LWhite,
                        Colors::White,
                        Colors::White,
                        Colors::Yellow,
                    ),
                ),
                (
                    'y',
                    ColorType::new(
                        "Yellow",
                        Colors::Yellow,
                        Colors::White,
                        Colors::Yellow,
                        Colors::LWhite,
                        Colors::LYellow,
                        Colors::LWhite,
                        Colors::White,
                        Colors::White,
                        Colors::Yellow,
                    ),
                ),
                (
                    'R',
                    ColorType::new(
                        "Light Red",
                        Colors::LRed,
                        Colors::White,
                        Colors::LRed,
                        Colors::LWhite,
                        Colors::Yellow,
                        Colors::Red,
                        Colors::Yellow,
                        Colors::Yellow,
                        Colors::LRed,
                    ),
                ),
                (
                    'G',
                    ColorType::new(
                        "Light Green",
                        Colors::LGreen,
                        Colors::White,
                        Colors::LGreen,
                        Colors::LWhite,
                        Colors::Yellow,
                        Colors::Blue,
                        Colors::Yellow,
                        Colors::Yellow,
                        Colors::LGreen,
                    ),
                ),
                (
                    'B',
                    ColorType::new(
                        "Light Blue",
                        Colors::LBlue,
                        Colors::White,
                        Colors::LBlue,
                        Colors::LWhite,
                        Colors::Yellow,
                        Colors::Blue,
                        Colors::Yellow,
                        Colors::Yellow,
                        Colors::LUmber,
                    ),
                ),
                (
                    'U',
                    ColorType::new(
                        "Light Umber",
                        Colors::LUmber,
                        Colors::White,
                        Colors::LUmber,
                        Colors::LWhite,
                        Colors::Yellow,
                        Colors::Umber,
                        Colors::Yellow,
                        Colors::Yellow,
                        Colors::LUmber,
                    ),
                ),
                (
                    'p',
                    ColorType::new(
                        "Purple",
                        Colors::Purple,
                        Colors::White,
                        Colors::LPurple,
                        Colors::Slate,
                        Colors::LPurple,
                        Colors::Slate,
                        Colors::LPurple,
                        Colors::LPurple,
                        Colors::LPurple,
                    ),
                ),
                (
                    'v',
                    ColorType::new(
                        "Violet",
                        Colors::Violet,
                        Colors::White,
                        Colors::LPurple,
                        Colors::Slate,
                        Colors::LPurple,
                        Colors::Slate,
                        Colors::LPurple,
                        Colors::LPurple,
                        Colors::LPurple,
                    ),
                ),
                (
                    't',
                    ColorType::new(
                        "Teal",
                        Colors::Teal,
                        Colors::White,
                        Colors::Blue,
                        Colors::Slate,
                        Colors::LTeal,
                        Colors::Slate,
                        Colors::LTeal,
                        Colors::LTeal,
                        Colors::LBlue,
                    ),
                ),
                (
                    'm',
                    ColorType::new(
                        "Mud",
                        Colors::Mud,
                        Colors::White,
                        Colors::Green,
                        Colors::Slate,
                        Colors::Mustard,
                        Colors::Slate,
                        Colors::Mustard,
                        Colors::Mustard,
                        Colors::Umber,
                    ),
                ),
                (
                    'Y',
                    ColorType::new(
                        "Light Yellow",
                        Colors::LYellow,
                        Colors::White,
                        Colors::Yellow,
                        Colors::White,
                        Colors::White,
                        Colors::Yellow,
                        Colors::White,
                        Colors::White,
                        Colors::LYellow,
                    ),
                ),
                (
                    'i',
                    ColorType::new(
                        "Magenta-Pink",
                        Colors::Magenta,
                        Colors::White,
                        Colors::LRed,
                        Colors::Slate,
                        Colors::LPink,
                        Colors::Red,
                        Colors::LPink,
                        Colors::LPink,
                        Colors::LPurple,
                    ),
                ),
                (
                    'T',
                    ColorType::new(
                        "Light Teal",
                        Colors::LTeal,
                        Colors::White,
                        Colors::LBlue,
                        Colors::LWhite,
                        Colors::Yellow,
                        Colors::Teal,
                        Colors::Yellow,
                        Colors::Yellow,
                        Colors::Blue,
                    ),
                ),
                (
                    'V',
                    ColorType::new(
                        "Light Violet",
                        Colors::LViolet,
                        Colors::White,
                        Colors::LPurple,
                        Colors::LWhite,
                        Colors::Yellow,
                        Colors::Violet,
                        Colors::Yellow,
                        Colors::Yellow,
                        Colors::LPurple,
                    ),
                ),
                (
                    'I',
                    ColorType::new(
                        "Light Pink",
                        Colors::LPink,
                        Colors::White,
                        Colors::LRed,
                        Colors::LWhite,
                        Colors::Yellow,
                        Colors::Magenta,
                        Colors::Yellow,
                        Colors::Yellow,
                        Colors::LPurple,
                    ),
                ),
                (
                    'M',
                    ColorType::new(
                        "Mustard",
                        Colors::Mustard,
                        Colors::White,
                        Colors::Yellow,
                        Colors::Slate,
                        Colors::Yellow,
                        Colors::Slate,
                        Colors::Yellow,
                        Colors::Yellow,
                        Colors::Yellow,
                    ),
                ),
                (
                    'z',
                    ColorType::new(
                        "Blue Slate",
                        Colors::BlueSlate,
                        Colors::White,
                        Colors::LWhite,
                        Colors::Slate,
                        Colors::DeepLBlue,
                        Colors::Slate,
                        Colors::DeepLBlue,
                        Colors::DeepLBlue,
                        Colors::LWhite,
                    ),
                ),
                (
                    'Z',
                    ColorType::new(
                        "Deep Light Blue",
                        Colors::DeepLBlue,
                        Colors::White,
                        Colors::LBlue,
                        Colors::LWhite,
                        Colors::LBlue,
                        Colors::BlueSlate,
                        Colors::LBlue,
                        Colors::LBlue,
                        Colors::LBlue,
                    ),
                ),
            ]),
            gamma_helper,
            gamma_table: [0; 256],
        }
    }

    pub fn char_to_attr(&self, c: &char) -> &Colors {
        match self.color_table.get(c) {
            Some(color) => &color.color_translate[&Attrs::Full],
            None => &Colors::White,
        }
    }

    pub fn text_to_attr(&self, name: &str) -> &Colors {
        match self.color_table.values().find(|v| v.name == name) {
            Some(color) => &color.color_translate[&Attrs::Full],
            None => &Colors::White,
        }
    }

    pub fn attr_to_text(&self, a: &Colors) -> &str {
        match self
            .color_table
            .values()
            .find(|v| v.color_translate[&Attrs::Full] == *a)
        {
            Some(color) => &color.name,
            None => "Icky",
        }
    }

    pub fn get_color(&self, a: &char, attr: &Attrs, n: i32) -> &Colors {
        todo!() // No idea what these arguments actually are - the source is unclear about char vs byte vs enum

        /***** source:
        if (a & (0x80))
            return (a);

        /* TODO: Honour the attribute for the term (full color, mono, 16 color) */
        if (!attr)
            return (a);

        /* Translate the color N times */
        while (n > 0) {
            a = color_table[a].color_translate[attr];
            n--;
        }

        /* Return the modified color */
        return (a);
        */
    }

    pub fn random(&self, rng: &mut Random) -> Colors {
        match rng.randint0(29) {
            0 => Colors::Dark,
            1 => Colors::White,
            2 => Colors::Slate,
            3 => Colors::Orange,
            4 => Colors::Red,
            5 => Colors::Green,
            6 => Colors::Blue,
            7 => Colors::Umber,
            8 => Colors::LDark,
            9 => Colors::LWhite,
            10 => Colors::LPurple,
            11 => Colors::Yellow,
            12 => Colors::LRed,
            13 => Colors::LGreen,
            14 => Colors::LBlue,
            15 => Colors::LUmber,
            16 => Colors::Purple,
            17 => Colors::Violet,
            18 => Colors::Teal,
            19 => Colors::Mud,
            20 => Colors::LYellow,
            21 => Colors::Magenta,
            22 => Colors::LTeal,
            23 => Colors::LViolet,
            24 => Colors::LPink,
            25 => Colors::Mustard,
            26 => Colors::BlueSlate,
            27 => Colors::DeepLBlue,
            28 => Colors::Shade,
            _ => Colors::Dark
        }
    }

    pub fn build_gamma_table(&mut self, gamma: i64) {
        self.gamma_table[0] = 0;
        self.gamma_table[255] = 255;

        for i in 1..255 {
            let mut n = 1;
            let mut value: i64 = 256 * 256;
            let mut diff: i64 = self.gamma_helper[i] * (gamma - 256_i64);

            while diff > 0 {
                value += diff;
                n += 1;

                diff = (((diff / 256) * self.gamma_helper[i]) * (gamma - 256)) / (256 * n);
            }

            self.gamma_table[i] = (((value / 256) * (i as i64)) / 256) as u8;
        }
    }
}
