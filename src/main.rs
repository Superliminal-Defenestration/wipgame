use bevy::prelude::*;

use wipgame::terminal::TerminalPlugin;

fn main() {
    let app = App::new().add_plugins(TerminalPlugin);
}
