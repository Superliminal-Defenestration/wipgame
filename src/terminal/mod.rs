use std::io::stdout;

use bevy::prelude::*;
use crossterm::{
    event::{self, read},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.set_runner(crate::terminal::terminal_runner);
    }
}

fn terminal_runner(mut app: App) {
    initiate_terminal();

    let mut should_close = false;

    while !should_close {
        if let Ok(event::Event::Key(key_event)) = read() {
            match key_event.code {
                event::KeyCode::Char('q') => should_close = true,
                _ => (),
            }
        }
    }

    cleanup_terminal();
}

fn initiate_terminal() {
    enable_raw_mode().unwrap();
    stdout()
        .execute(EnterAlternateScreen)
        .expect("Failed to enter alternate screen");
}

fn cleanup_terminal() {
    disable_raw_mode().unwrap();

    stdout()
        .execute(LeaveAlternateScreen)
        .expect("Failed to exit alternate screen");
}
