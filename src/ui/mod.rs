pub mod game;
pub mod menu;

pub enum Screen {
    GameScreen (game::GameScreen),
    Menu (menu::Menu)
}
