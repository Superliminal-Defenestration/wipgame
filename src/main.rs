use bevy::prelude::*;

use wipgame::terminal::TerminalPlugin;

#[derive(States, Debug, Eq, Hash, Clone, PartialEq, Default)]
pub enum GameState {
    MainMenu,
    #[default]
    Game,
}

fn main() {
    let app = App::new().add_plugins(TerminalPlugin).run();
}
