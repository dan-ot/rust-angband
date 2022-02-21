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
    let tilesets = ui::graphics::GraphicsModeService::from_folders(Path::new("resources/tiles"), Path::new("resources/fonts"));

    let context = sdl2::init().unwrap();
    let font_context = sdl2::ttf::init().unwrap();

    let mut engine = engine::Engine::new(&context);
    engine.run()
}
