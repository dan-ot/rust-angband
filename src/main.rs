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
    let context = sdl2::init().unwrap();
    let mut engine = engine::Engine::new(&context);

    engine.run()
}
