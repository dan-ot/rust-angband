mod events;
mod types;
mod random;
mod objects;
mod bitflags;
mod expressions;
mod dice;
mod player;
mod monsters;
mod colors;
mod messages;
mod cave;
mod ui;
mod engine;

fn main() {
    let context = sdl2::init().unwrap();
    let mut engine = engine::Engine::new(&context);

    engine.run()
}
