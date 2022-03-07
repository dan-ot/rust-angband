#![warn(clippy::all)]

use std::path::Path;

mod bitflags;
mod cave;
mod colors;
mod dice;
mod engine;
mod events;
mod expressions;
mod messages;
mod monsters;
mod objects;
mod player;
mod random;
mod types;
mod ui;

fn main() {
    let graphics = ui::graphics::GraphicsModeService::from_folders(Path::new("resources/tiles"), Path::new("resources/fonts"));

    let context = sdl2::init().unwrap();
    let font_context = sdl2::ttf::init().unwrap();

    let video_subsystem = context.video().unwrap();
    let mode = video_subsystem.current_display_mode(0).unwrap();
    let window = video_subsystem
        .window(
            "rust-angband",
            (mode.w as f32 * 0.8) as u32,
            (mode.h as f32 * 0.8) as u32,
        )
        .maximized()
        .build()
        .unwrap();

    let canvas = Box::new(window.into_canvas().build().unwrap());

    let mut engine = engine::Engine::new(&context, canvas, font_context, graphics);
    engine.run()
}
