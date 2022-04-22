use std::convert::TryInto;
use image::RgbImage;

fn from_word(s: &[u8]) -> u16 {
    let l = s[0];
    let r = s[1];
    (l as u16) + 256 * (r as u16)
}

fn from_dword(s: &[u8]) -> u32 {
    let l = from_word(&s[0..2]);
    let r = from_word(&s[2..4]);
    (l as u32) | ((r as u32) << 16)
}

fn load_cp1252(fnt: &[u8]) -> Vec<RgbImage> {
    let version = from_word(&fnt[0..]);
    let ftype = from_word(&fnt[0x42..]);

    if ftype & 1 != 0 {
        panic!("This font is a vector font.");
    }
    let facename_offset: usize = from_dword(&fnt[0x69..]).try_into().unwrap();
    if /* facename_offset < 0 || */ facename_offset > fnt.len() {
        panic!("Face name not contained within font data")
    }
    // let facename = std::str::from_utf8(&fnt[facename_offset..]).unwrap();
    // let copyright = std::str::from_utf8(&fnt[6..66]).unwrap();
    // let pointsize = from_word(&fnt[0x44..]);
    // let ascent = from_word(&fnt[0x4A..]);
    // let inleading = from_word(&fnt[0x4C..]);
    // let exleading = from_word(&fnt[0x4E..]);
    let height = from_word(&fnt[0x58..]);
    // let italic = fnt[0x50] != 0;
    // let underline = fnt[0x51] != 0;
    // let strikeout = fnt[0x52] != 0;
    // let weight = fnt[0x53] != 0;
    // let charset = fnt[0x55];

    let (ctstart, ctsize) = match version {
        0x200 => (0x76_u32, 4_u32),
        _ => (0x94_u32, 6_u32)
    };
    let mut max_width = 0;
    let first_char = fnt[0x5f] as u32;
    let last_char = fnt[0x60] as u32;

    (first_char..=last_char).map(|i| {
        let entry: usize = (ctstart + ctsize * (i - first_char)).try_into().unwrap();
        let width = from_word(&fnt[entry..]);
        max_width = std::cmp::max(width, max_width);
        let off = match ctsize {
            4 => from_word(&fnt[entry + 2..]) as u32,
            _ => from_dword(&fnt[entry + 2..])
        };
        let widthbytes = (width + 7) / 8;
        let mut im = image::RgbImage::new(width as u32, height as u32);
        for j in 0..height {
            for k in 0..widthbytes {
                let bytepos: usize = (off + (k as u32) * (height as u32) + (j as u32)).try_into().unwrap();
                let b = fnt[bytepos];
                for n in 0..8 {
                    if b & (1 << n) != 0 {
                        im.put_pixel(7 - n, j as u32, image::Rgb([255, 255, 255]));
                    }
                }
            }
        }
        im
    }).collect()
}

fn extract_ne_fonts(fon: &[u8], neoff: usize) -> Vec<Vec<RgbImage>> {
    let rtable_offset: usize = from_word(&fon[neoff + 0x24..]).try_into().unwrap();
    let rtable_header = rtable_offset + neoff;
    let shift = from_word(&fon[rtable_header..]);
    let mut p = rtable_header + 2;
    let mut fonts = vec!();
    loop {
        let rtype = from_word(&fon[p..]);
        if rtype == 0 {
            break;
        }
        let count = from_word(&fon[p + 2..]);
        println!("{} fonts found", count);
        p += 8; // skip type (2 bytes), the count we just read (2 bytes), and 4 reserved bytes
        for _ in [0..count] {
            let start: usize = (from_word(&fon[p..]) << shift).try_into().unwrap();
            let size: usize = (from_word(&fon[p + 2..]) << shift).try_into().unwrap();
            if (start + size) as usize > fon.len() {
                panic!("Resource overrun!");
            }
            if rtype == 0x8008 {
                fonts.push(load_cp1252(&fon[start..start + size]));
            }
            p += 12 // start (2 bytes), size (2 bytes), flags, name/id, 4 bytes reserved
        }
    }
    fonts
}

pub fn load_fonts(fon: &[u8]) -> Vec<Vec<RgbImage>> {
    if [fon[0], fon[1]] != *b"MZ" {
        panic!("Not a .FON format!");
    } else {
        let neoff: usize = from_dword(&fon[0x3C..]).try_into().unwrap();
        if fon[neoff..(neoff + 2)] == *b"NE" {
            extract_ne_fonts(fon, neoff)
        } else if fon[neoff..(neoff + 4)] == *b"PE\0\0" {
            println!("PE!");
            vec!()
        } else {
            panic!("Couldn't find an offset (NE or PE).");
        }
    }
}