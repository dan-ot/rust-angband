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
    let tilesets = ui::graphics::GraphicsModeService::from_folder(Path::new("resources/tiles"));

    for tile in tilesets.graphics_modes.iter() {
        println!("{}: {:?}", tile.menuname, tile.path)
    }

    println!("Tilesets found.");
    let context = sdl2::init().unwrap();
    let mut engine = engine::Engine::new(&context);
    engine.run()
}
