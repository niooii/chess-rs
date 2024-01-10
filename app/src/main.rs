use std::{sync::Arc, time::Duration, collections::HashMap};

use chess::{self, board::Board, game::{Game, self}};
use debug_ui::App;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode, render::Texture};

mod debug_ui;

pub fn main() -> Result<(), String> {
    let game = Game::original().unwrap();
    let mut app = App::new(game)?;

    // should be stored in the following key format:
    // ["TEAM:PIECE"]
    let mut texture_map: HashMap<String, Texture> = HashMap::new();

    app.init_texturemap(&mut texture_map);

    app.run(&texture_map);

    Ok(())
}