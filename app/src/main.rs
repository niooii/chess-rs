use std::{sync::Arc, time::Duration};

use chess::{self, board::Board, game::{Game, self}};
use debug_ui::App;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode};

mod debug_ui;

pub fn main() -> Result<(), String> {
    let game = Game::original().unwrap();
    let mut app = App::new(game)?;

    app.run();

    Ok(())
}