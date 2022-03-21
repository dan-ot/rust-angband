#![warn(clippy::all)]
#![windows_subsystem="windows"]
extern crate glfw;

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
mod glad_gl;

fn main() {
    let graphics = ui::graphics::GraphicsModeService::from_folders(Path::new("resources/tiles"), Path::new("resources/fonts"));

    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let mut engine = engine::Engine::new(glfw, graphics);
    engine.run()
}
