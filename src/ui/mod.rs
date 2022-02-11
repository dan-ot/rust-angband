use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::ttf::Font as SDLFont;
use sdl2::video::Window as SDLWindow;
use std::collections::VecDeque;

pub mod ui_event;

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

pub struct WindowConfig<'a> {
    // TODO: Turn these flags into HashSets...
    pub renderer_flags: i32,
    pub renderer_index: i32,
    pub window_flags: i32,

    // TODO: Do we actually need these paths? Do we load the file more than once,
    // or are these paths here because of order-of-operations on init?
    pub wallpaper_path: &'a str,
    pub font_name: &'a str,
    pub font_size: i32,
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
    pub color: Color,
    pub width: i32,
}

pub enum WallpaperMode {
    Invalid,
    DontShow,
    Tiled,
    Centered,
    Scaled,
}

pub struct Wallpaper<'a> {
    pub w: i32,
    pub h: i32,
    pub texture: Texture<'a>,
    pub mode: WallpaperMode,
}

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

pub struct Ttf<'a, 'b> {
    pub handle: SDLFont<'a, 'b>,
    pub glyph: SizeTuple,
}

/// I think this is the in-memory sprite sheet for a font, and a list of the rects for each character.
/// If this is the case, it's probably a char->Rect map instead of a Vec
pub struct FontCache<'a> {
    pub texture: Texture<'a>,
    pub rects: Vec<Rect>,
}

pub struct Font<'a, 'b> {
    pub ttf: Ttf<'a, 'b>,
    pub name: &'a str,
    pub path: &'a str,

    pub size: i32,
    /// Index of font in global font array?
    pub index: isize,

    pub cache: FontCache<'a>,
}

pub struct TermFlagValue<'a, 'b, U, D> {
    pub subwindow: &'a Subwindow<'a, 'b, U, D>,
    pub flag: u32,
}

pub struct AlphaValue<'a, 'b, U, D> {
    pub subwindow: &'a Subwindow<'a, 'b, U, D>,
    pub real_value: i32,
    pub show_value: i32,
}

// TODO: As you can see from the <'a, U, D> proliferation, we're borrowing subwindows
// Does it make sense to index into a set instead? This feel backward.
pub enum ButtonData<'a, 'b, U, D> {
    Invalid,
    None,
    Int(i32),
    Unsigned(u32),
    Subwindow(&'a Subwindow<'a, 'b, U, D>),
    Font(&'a Font<'a, 'b>),
    TermFlag(&'a TermFlagValue<'a, 'b, U, D>),
    Alpha(&'a AlphaValue<'a, 'b, U, D>),
}

pub struct ButtonCallbacks<'a, 'b, U, D> {
    pub on_render: Box<dyn FnMut(&'a Window<'a, 'b, U, D>, &'a mut Button<'a, 'b, U, D>) -> ()>,
    pub on_event:
        Box<dyn FnMut(&'a mut Window<'a, 'b, U, D>, &'a mut Button<'a, 'b, U, D>, &Event) -> bool>,
    pub on_menu: Box<
        dyn FnMut(
            &'a mut Window<'a, 'b, U, D>,
            &'a mut Button<'a, 'b, U, D>,
            &mut Event,
            &'a mut MenuPanel<'a, 'b, U, D>,
        ) -> (),
    >,
}

pub struct Button<'a, 'b, U, D> {
    /// Selected means pressed but not released (mid-activation)
    pub selected: bool,
    /// Highlighted means focused/hovered but not pressed
    pub highlighted: bool,

    pub caption: &'a str,

    pub full_rect: Rect,
    pub inner_rect: Rect,

    pub data: ButtonData<'a, 'b, U, D>,
    // TODO: Instead of function pointers and callbacks, register events for the event pipe?
    pub callbacks: ButtonCallbacks<'a, 'b, U, D>,
}

pub struct ButtonBank<'a, 'b, U, D> {
    pub buttons: Vec<Button<'a, 'b, U, D>>,

    // TODO: What's the difference between size and number?
    pub size: isize,
    pub number: isize,
}

// TODO: This is also a linked list. Need to find consumers and change them, rather than optionally not linking
pub struct MenuPanel<'a, 'b, U, D> {
    pub rect: Rect,
    pub button_bank: ButtonBank<'a, 'b, U, D>,
    pub next: &'a MenuPanel<'a, 'b, U, D>,
}

pub struct StatusBar<'a, 'b, U, D> {
    pub font: Font<'a, 'b>,

    pub button_bank: ButtonBank<'a, 'b, U, D>,
    pub menu_panel: MenuPanel<'a, 'b, U, D>,

    // TODO: This set is on the Window as well. Is this a separate type composed into these?
    // Is this actually a copy of the ones on the window, or derived?
    pub full_rect: Rect,
    pub inner_rect: Rect,
    pub color: Color,
    pub texture: Texture<'a>,

    pub in_menu: bool,
}

pub struct Graphics<'a> {
    pub texture: Texture<'a>,

    pub id: i32,
    pub tile_pixel_w: i32,
    pub tile_pixel_h: i32,

    pub overdraw_row: i32,
    pub overdraw_max: i32,
}

pub struct Subwindow<'a, 'b, U, D> {
    pub inited: bool,
    pub loaded: bool,
    pub linked: bool,
    pub visible: bool,

    pub config: SubwindowConfig<'a>,

    pub top: bool,
    pub always_top: bool,

    pub index: u32,

    pub rows: i32,
    pub cols: i32,

    // TODO: These are cached here because... (their source of truth is the font field)
    pub font_width: i32,
    pub font_height: i32,

    /// Coordinates of full_rect are relative to coordinates of window
    /// (basically, full rect is texture)
    pub full_rect: Rect,
    /// Coordinates of inner_rect are relative to full_rect
    pub inner_rect: Rect,
    /// For use when resizing
    pub sizing_rect: Rect,
    /// A one-pixel texture, mostly for filling with something while the player is resizing
    pub aux_texture: Texture<'a>,

    /// Background color
    pub color: Color,

    pub borders: SubwindowBorder,
    pub texutre: Texture<'a>,
    pub font: Font<'a, 'b>,

    // TODO: Are these circular references?
    pub winow: Window<'a, 'b, U, D>,
    pub term: Terminal<U, D>,
}

/// A real window on screen, with one or more subwindows in it
pub struct Window<'a, 'b, U, D> {
    pub inited: bool,
    pub loaded: bool,

    /// SDL's Id, for use with events
    pub id: u32,
    /// Our internal id, mostly for debugging
    pub index: u16,

    pub config: WindowConfig<'a>,

    /// Has mouse focus
    pub focus: bool,
    /// Needs to be redrawn
    pub dirty: bool,

    /// Framerate limiter
    pub limiter: u32,
    /// From display mode
    pub delay: i32,

    /// Reported by SDL_GetWindowFlags()
    pub flags: u32,

    /// Position and size of the window on display
    pub full_rect: Rect,
    /// Size of window without status bar
    pub inner_rect: Rect,

    pub color: Color,

    pub window: SDLWindow,
    // This used to be an SDL_Renderer, but I think SDL moved on
    pub renderer: Canvas<SDLWindow>,

    pub pixelformat: i16,

    pub wallpaper: Wallpaper<'a>,
    pub move_state: MoveState,
    pub size_state: SizeState,
    pub status_bar: StatusBar<'a, 'b, U, D>,
    pub graphics: Graphics<'a>,

    pub subwindows: Vec<Subwindow<'a, 'b, U, D>>,
}
