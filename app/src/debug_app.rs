use chess::game::Game;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::image::{LoadTexture, InitFlag};
use std::collections::HashMap;
use std::time::Duration;

pub struct App {
    pub game: Game,
    pub running: bool
}

impl App {
    pub fn new(game: Game) -> Result<Self, String> {
        Ok(App {
            game,
            running: true
        })
    }

    pub fn handle_event(&mut self, event_pump: &mut EventPump) {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.running = false,
                _ => {}
            }
        }
    }

    pub fn init_texturemap(&mut self, texture_creator: &mut TextureCreator<WindowContext>, texture_map: &mut HashMap<String, Texture>) {
        
    }

    pub fn render_game(&mut self, canvas: &mut Canvas<Window>, texture_map: &HashMap<String, Texture>) {
        const TILE_WIDTH: u32 = 50;
        const TILE_HEIGHT: u32 = 50;
        let primary_color = Color::RGBA(0, 255, 0, 255);
        let secondary_color = Color::RGBA(255, 255, 255, 255);

        let board = self.game.board();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for y in 0..board.height() {
            for x in 0..board.width() {
                canvas.set_draw_color(
                    if x % 2 == 0 {
                        if y % 2 == 0 {
                            primary_color
                        } else {
                            secondary_color
                        }
                    } else if y % 2 == 1 {
                        primary_color
                    } else {
                        secondary_color
                    }
                );
                let tile_rect = Rect::new((x * TILE_WIDTH) as i32, ((board.height() - 1 - y) * TILE_HEIGHT) as i32, TILE_WIDTH, TILE_HEIGHT);
                canvas.fill_rect(tile_rect).unwrap();
                let tile = board.tile_at(x, y).unwrap();
                let tile_rlock = tile.read().unwrap();
                let piece = tile_rlock.piece();

                if let Some(p) = piece {
                    let piece_rlock = p.read().unwrap();
                    
                    let key = format!("{}:{}", piece_rlock.team_unchecked().name(), piece_rlock.name());
                    // println!("{key}");
                    canvas.copy(&texture_map[&key], None, tile_rect).unwrap();
                }
            }
        }

        for game_move in self.game.calculate_moves_for("White".to_string()).unwrap() {
            let to = game_move.to();

            let tile_rect = Rect::new((to.x() * TILE_WIDTH) as i32, ((board.height() - 1 - to.y()) * TILE_HEIGHT) as i32, TILE_WIDTH, TILE_HEIGHT);

            canvas.set_draw_color(Color::RED);
            canvas.fill_rect(tile_rect).unwrap();
        }

        canvas.present();
    }
}