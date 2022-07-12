// use sdl2::event::Event;
// use sdl2::pixels::Color;
// use sdl2::rect::Rect;
// use sdl2::render::{Canvas, Texture, WindowCanvas, TextureAccess, BlendMode, TextureCreator};
// use sdl2::ttf::{Font as SDLFont, Sdl2TtfContext, Hinting};
// use sdl2::video::{Window as SDLWindow, WindowContext};
use std::collections::VecDeque;
use std::convert::TryFrom;

pub mod ui_event;
pub mod graphics;
pub mod tileset;
pub mod chars;
pub mod fon;

// TODO: in the source, these are a linked list...
pub struct TerminalWindow {
    /// Cursor useless
    pub cu: bool,
    /// Cursor visible
    pub cv: bool,

    // Cursor Location
    pub cx: i32,
    pub cy: i32,
    pub cnx: i32,
    pub cny: i32,

    // TODO: these could probably be tuples
    /// Attributes (2-dimensional)
    pub a: Vec<Vec<i32>>,
    /// Chars (2-dimensional)
    pub c: Vec<Vec<char>>,

    // TODO: visual attr/char?
    pub va: Vec<i32>,
    pub vc: Vec<char>,

    // TODO: temp 2-dimensionals?
    pub ta: Vec<Vec<i32>>,
    pub tc: Vec<Vec<char>>,

    // TODO: Visual temp arrays?
    pub vta: Vec<i32>,
    pub vtc: Vec<char>,
}

pub struct Terminal<U, D> {
    // TODO: Using generics here because I don't know what types these might end up being
    pub user: U,
    pub data: D,
    // TODO: These are also part of the data above, probably need to be an enum with tuple data
    pub user_flag: bool,
    pub data_flag: bool,

    pub active_flag: bool,
    pub mapped_flag: bool,
    pub total_erase: bool,
    pub fixed_shape: bool,
    // pub icky_corner: bool, <-- always false on our platforms
    pub soft_cursor: bool, // Always true?
    /// Use `Term_pict()` to draw text
    pub always_pict: bool,
    /// Use `Term_pict()` for special text
    pub higher_pict: bool,
    /// Use `Term_text()` for invisible text
    pub always_text: bool,
    /// Don't emit/handle `TERM_XTRA_BORED`
    pub never_bored: bool,
    /// Don't emit/handle `TERM_XTRA_FROSH`
    pub never_frosh: bool,
    pub sidebar_mode: i32,

    /// Use this `attr` for 'blank' grids
    pub attr_blank: i32,
    /// Use this `char` for 'blank' grids
    pub char_blank: char,

    /// Distinguish between Enter/^m/^j, etc.
    pub complex_input: bool,

    pub key_queue: VecDeque<ui_event::ui_event>,
    pub key_xtra: u16,

    // TODO: Should these values be Tuples, or some engine-level construct like sdl::Rect?
    /// Width (max 255)
    pub wid: i32,
    /// Height (max 255)
    pub hgt: i32,

    /// Minimum modified row
    pub y1: i32,
    /// Maximum modified row
    pub y2: i32,

    /// Minimum modified column (per modified row)
    pub x1: Vec<i32>,
    /// Maximum modified column (per modified row)
    pub x2: Vec<i32>,
    // The length of these Vecs is equal to y2 - y1 (the number of modified rows)
    // Generally, we're tracking one dirty rectangle per row - probably because of the
    // paradigm of rendering rows of equal-sized text...
    /// Offset for the map subwindows
    pub offset_x: i32,
    /// Offset for the map subwindows
    pub offset_y: i32,

    /// Displayed screen image
    pub old: TerminalWindow,
    /// Requested screen image
    pub new: TerminalWindow,

    /// Temporary screen image
    pub tmp: TerminalWindow,
    /// Memorized screen image
    pub mem: TerminalWindow,

    /// Number of times saved
    pub saved: u8,
}

impl<U, D> Terminal<U, D> {
    fn init_hook(&mut self) {}

    fn nuke_hook(&mut self) {}

    fn xtra_hook(&mut self) {}

    fn curs_hook(&mut self) {}

    fn bigcurs_hook(&mut self) {}

    fn wipe_hook(&mut self) {}

    fn text_hook(&mut self) {}

    fn pict_hook(&mut self) {}

    fn view_map_hook(&mut self) {}

    fn dblh_hook(&mut self) {}
}

pub struct WindowConfig {
    // TODO: Turn these flags into HashSets...
    pub renderer_flags: u32,
    pub renderer_index: usize,
    pub window_flags: u32,

    // TODO: Do we actually need these paths? Do we load the file more than once,
    // or are these paths here because of order-of-operations on init?
    pub wallpaper_path: String,
    pub font_name: String,
    pub font_size: u8,
}

pub struct SubwindowConfig<'a> {
    // TODO: Do we actually need this? Do we end up reloading the font from these, or is
    // this just a builder artifact that sticks around?
    pub font_name: &'a str,
    pub font_size: i32,
}

pub struct SubwindowBorder {
    pub visible: bool,
    pub error: bool,
    // pub color: Color,
    pub width: i32,
}

pub enum WallpaperMode {
    Invalid,
    DontShow,
    Tiled,
    Centered,
    Scaled,
}

// pub struct Wallpaper<'a> {
//     pub w: i32,
//     pub h: i32,
//     pub texture: Texture<'a>,
//     pub mode: WallpaperMode,
// }

/// State-tracking for dragging terminals
pub struct MoveState {
    pub active: bool,
    pub moving: bool,

    pub originx: i32,
    pub originy: i32,
}

/// State-tracking for resizing terminals
pub struct SizeState {
    pub active: bool,
    pub sizing: bool,

    pub originx: i32,
    pub originy: i32,

    pub left: bool,
    pub top: bool,
}

pub struct SizeTuple {
    pub w: i32,
    pub h: i32,
}

// pub struct Ttf<'a, 'b> {
//     pub handle: SDLFont<'a, 'b>,
//     pub glyph: SizeTuple,
// }

// /// I think this is the in-memory sprite sheet for a font, and a list of the rects for each character.
// /// If this is the case, it's probably a char->Rect map instead of a Vec. As it stands, the index in this vec
// /// matches the index in the global list of 'useable characters'
// pub struct FontAtlas<'a> {
//     /// A blittable source for font glyphs. All pure-white, for use in blending as tiles or actual UI fontage.
//     pub texture: Texture<'a>,
//     /// The set of Rects describing each blittable glyph.
//     pub rects: Vec<Rect>,
// }

// impl FontAtlas<'_> {
//     pub fn render_atlas<'a>(font: &graphics::FontInfo, font_context: &Sdl2TtfContext, canvas: &'a mut WindowCanvas, texture_creator: &'a TextureCreator<WindowContext>) -> FontAtlas<'a> {
//         let font_size = match font.font_type {
//             // For Raster fonts, size is an offset
//             graphics::FontType::Raster => 0,
//             // For Vector fonts, size is a measure. At some point, we'll make this more adjustable via configs or inputs
//             graphics::FontType::Vector => 12
//         };
//         let mut font_handle = font_context.load_font(&font.path, font_size).unwrap();
//         font_handle.set_hinting(Hinting::Light);
//         // Assumption: Font metrics won't have negative height or width despite returning as i32.
//         let height = u32::try_from(font_handle.height()).unwrap();
//         // Assumption: Capital 'W' is the widest glyph for a given font. This will allow us to create a monospace font by centering any narrower glyphs.
//         // 'advance' as a field provides a little extra room around the edges. This may not be exactly what we want, if we're trying to use a solid block for walls. Or maybe that works fine for those glyphs.
//         // However, I'm not sure this carries through to full CP437 (consider some of the box-drawing glpyhs)
//         let target_width = u32::try_from(font_handle.find_glyph_metrics('W').unwrap().advance).unwrap();
//         let mut texture = texture_creator.create_texture_target(None, target_width * u32::try_from(cp437::ASCII.len()).unwrap(), height).unwrap();
//         texture.set_blend_mode(BlendMode::Blend);
        
//         let mut rects: Vec<Rect> = vec!();
//         canvas.with_texture_canvas(&mut texture, |mut_c| {
//             for (idx, glyph) in cp437::ASCII.char_indices() {
//                 println!("Loading font {}", font.name);
//                 let rendered_texture = font_handle.render(&glyph.to_string()).blended(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
//                 let metrics = font_handle.find_glyph_metrics(glyph).unwrap();
//                 let glyph_width = u32::try_from(metrics.maxx - metrics.minx).unwrap();
//                 let left_padding = metrics.minx;
//                 let src = Rect::new(
//                     // Skip the normally-rendered space to the left of the glyph
//                     left_padding, 
//                     // Start at the top (bottom?)
//                     0,
//                     // don't include the space to the right of the glpyh
//                     glyph_width,
//                     // Take the whole height
//                     height
//                 );
//                 let dst = Rect::new(
//                     // We're monospacing, so offset by the index
//                     i32::try_from(idx).unwrap() * i32::try_from(target_width).unwrap()
//                     // Pad for half the width of the actual glyph
//                         + i32::try_from(glyph_width / 2).unwrap(),
//                     // Start at the top (bottom?)
//                     0, 
//                     // We need to center the glyph (not the advance) inside our monospace standard determined by 'W'
//                     // If we supply a width larger than the glyph, it will be stretched (and uncentered)
//                     glyph_width, 
//                     // Write the whole height.
//                     height
//                 );
//                 mut_c.copy(&rendered_texture, src, dst).unwrap();
//                 rects.push(dst);
//                 print!("{} @ [{}/{}]", glyph, dst.left(), dst.width())
//             }
//         }).unwrap();

//         FontAtlas::<'a> {
//             texture,
//             rects
//         }
//     }
// }

// pub struct Font<'a, 'b> {
//     // TODO: Why do we need these? The font is loaded.
//     pub name: String,
//     pub path: String,
//     pub ttf: Ttf<'a, 'b>,
    
//     /// Point size for a vector font
//     pub size: u8,
//     /// Index of font in global font array?
//     pub index: isize,
    
//     pub cache: FontAtlas<'a>,
// }

// pub struct TermFlagValue<'a, 'b, U, D> {
//     pub subwindow: &'a Subwindow<'a, 'b, U, D>,
//     pub flag: u32,
// }

// pub struct AlphaValue<'a, 'b, U, D> {
//     pub subwindow: &'a Subwindow<'a, 'b, U, D>,
//     pub real_value: i32,
//     pub show_value: i32,
// }

// TODO: As you can see from the <'a, U, D> proliferation, we're borrowing subwindows
// Does it make sense to index into a set instead? This feel backward.
// pub enum ButtonData<'a, 'b, U, D> {
//     Invalid,
//     None,
//     Int(i32),
//     Unsigned(u32),
//     Subwindow(&'a Subwindow<'a, 'b, U, D>),
//     Font(&'a Font<'a, 'b>),
//     TermFlag(&'a TermFlagValue<'a, 'b, U, D>),
//     Alpha(&'a AlphaValue<'a, 'b, U, D>),
// }

// pub struct ButtonCallbacks<'a, 'b, U, D> {
//     pub on_render: Box<dyn FnMut(&'a Window<'a, 'b, U, D>, &'a mut Button<'a, 'b, U, D>) -> ()>,
//     pub on_event:
//         Box<dyn FnMut(&'a mut Window<'a, 'b, U, D>, &'a mut Button<'a, 'b, U, D>, &Event) -> bool>,
//     pub on_menu: Box<
//         dyn FnMut(
//             &'a mut Window<'a, 'b, U, D>,
//             &'a mut Button<'a, 'b, U, D>,
//             &mut Event,
//             &'a mut MenuPanel<'a, 'b, U, D>,
//         ) -> (),
//     >,
// }

// pub struct Button<'a, 'b, U, D> {
//     /// Selected means pressed but not released (mid-activation)
//     pub selected: bool,
//     /// Highlighted means focused/hovered but not pressed
//     pub highlighted: bool,

//     pub caption: &'a str,

//     pub full_rect: Rect,
//     pub inner_rect: Rect,

//     pub data: ButtonData<'a, 'b, U, D>,
//     // TODO: Instead of function pointers and callbacks, register events for the event pipe?
//     pub callbacks: ButtonCallbacks<'a, 'b, U, D>,
// }

// pub struct ButtonBank<'a, 'b, U, D> {
//     pub buttons: Vec<Button<'a, 'b, U, D>>,

//     // TODO: What's the difference between size and number?
//     pub size: isize,
//     pub number: isize,
// }

// TODO: This is also a linked list. Need to find consumers and change them, rather than optionally not linking
// pub struct MenuPanel<'a, 'b, U, D> {
//     pub rect: Rect,
//     pub button_bank: ButtonBank<'a, 'b, U, D>,
//     pub next: &'a MenuPanel<'a, 'b, U, D>,
// }

// pub struct StatusBar<'a, 'b, U, D> {
//     pub font: Font<'a, 'b>,

//     pub button_bank: ButtonBank<'a, 'b, U, D>,
//     pub menu_panel: MenuPanel<'a, 'b, U, D>,

//     // TODO: This set is on the Window as well. Is this a separate type composed into these?
//     // Is this actually a copy of the ones on the window, or derived?
//     pub full_rect: Rect,
//     pub inner_rect: Rect,
//     pub color: Color,
//     pub texture: Texture<'a>,

//     pub in_menu: bool,
// }

// pub struct Graphics<'a> {
//     pub texture: Texture<'a>,

//     pub id: i32,
//     pub tile_pixel_w: i32,
//     pub tile_pixel_h: i32,

//     pub overdraw_row: i32,
//     pub overdraw_max: i32,
// }

// pub struct Subwindow<'a, 'b, U, D> {
//     pub inited: bool,
//     pub loaded: bool,
//     pub linked: bool,
//     pub visible: bool,

//     pub config: SubwindowConfig<'a>,

//     pub top: bool,
//     pub always_top: bool,

//     pub index: u32,

//     pub rows: i32,
//     pub cols: i32,

//     // TODO: These are cached here because... (their source of truth is the font field)
//     pub font_width: i32,
//     pub font_height: i32,

//     /// Coordinates of full_rect are relative to coordinates of window
//     /// (basically, full rect is texture)
//     pub full_rect: Rect,
//     /// Coordinates of inner_rect are relative to full_rect
//     pub inner_rect: Rect,
//     /// For use when resizing
//     pub sizing_rect: Rect,
//     /// A one-pixel texture, mostly for filling with something while the player is resizing
//     pub aux_texture: Texture<'a>,

//     /// Background color
//     pub color: Color,

//     pub borders: SubwindowBorder,
//     pub texutre: Texture<'a>,
//     pub font: Font<'a, 'b>,

//     // TODO: Are these circular references?
//     pub winow: Window<'a, 'b, U, D>,
//     pub term: Terminal<U, D>,
// }

// /// A real window on screen, with one or more subwindows in it
// pub struct Window<'a, 'b, U, D> {
//     pub inited: bool,
//     pub loaded: bool,

//     /// SDL's Id, for use with events
//     pub id: u32,
//     /// Our internal id, mostly for debugging
//     pub index: u16,

//     pub config: WindowConfig,

//     /// Has mouse focus
//     pub focus: bool,
//     /// Needs to be redrawn
//     pub dirty: bool,

//     /// Framerate limiter
//     pub limiter: u32,
//     /// From display mode
//     pub delay: i32,

//     /// Reported by SDL_GetWindowFlags()
//     pub flags: u32,

//     /// Position and size of the window on display
//     pub full_rect: Rect,
//     /// Size of window without status bar
//     pub inner_rect: Rect,

//     pub color: Color,

//     pub window: SDLWindow,
//     // This used to be an SDL_Renderer, but I think SDL moved on
//     pub renderer: Canvas<SDLWindow>,

//     pub pixelformat: i16,

//     pub wallpaper: Wallpaper<'a>,
//     pub move_state: MoveState,
//     pub size_state: SizeState,
//     pub status_bar: StatusBar<'a, 'b, U, D>,
//     pub graphics: Graphics<'a>,

//     pub subwindows: Vec<Subwindow<'a, 'b, U, D>>,
// }
