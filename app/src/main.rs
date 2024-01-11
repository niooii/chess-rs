use std::{sync::Arc, time::Duration, collections::HashMap};

use chess::{self, board::Board, game::{Game, self}, piece};
use debug_app::App;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode, render::Texture, image::{InitFlag, LoadTexture}};

mod debug_app;

pub fn main() -> Result<(), String> {
    // SDL2
    let sdl_context = sdl2::init()?;
    let image = sdl2::image::init(InitFlag::all())?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("chess v3434", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let tex_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump()?;
    let game = Game::two_piece_test(piece::defaults::pawn()).unwrap();
    let mut app = App::new(game)?;

    // should be stored in the following key format:
    // ["TEAM:PIECE"]
    let mut texture_map: HashMap<String, Texture> = HashMap::new();

    
    // initialize textures
    let board = app.game.board();

        for y in (0..board.height()) {
            for x in 0..board.width() {
                let tile = board.tile_at(x, y).unwrap();
                let tile_rlock = tile.read().unwrap();
                let piece = tile_rlock.piece();

                if let Some(p) = piece {
                    let piece_rlock = p.read().unwrap();
                    
                    let path = format!("app/textures/{}/{}.png", piece_rlock.team_unchecked().name(), piece_rlock.name());
                    println!("{path}");
                    let key = format!("{}:{}", piece_rlock.team_unchecked().name(), piece_rlock.name());

                    if texture_map.contains_key(&key) {
                        continue;
                    }

                    let tex = tex_creator.load_texture(path).unwrap();

                    texture_map.insert(key, 
                        tex
                    );
                }
            }
        }

    let running = true;

    'mainapp: while app.running {
        app.handle_event(&mut event_pump);
        app.render_game(&mut canvas, &texture_map);
    }

    Ok(())
}