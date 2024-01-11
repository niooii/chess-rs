use chess::game::Game;
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
    game: Game,
    last_move: u32,
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
    texture_creator: TextureCreator<WindowContext>
    
}

impl App {
    pub fn new(game: Game) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let image = sdl2::image::init(InitFlag::all())?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let tex_creator = canvas.texture_creator();
        let event_pump = sdl_context.event_pump()?;
        
        Ok(App {
            game,
            last_move: u32::MAX,
            canvas,
            texture_creator: tex_creator, 
            event_pump,
        })
    }

    pub fn run(&mut self, texture_map: &HashMap<String, Texture>) {
        let mut running = true;

        while running {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => running = false,
                    _ => {}
                }
            }

            // // update game screen only when needed
            // if self.last_move != self.game.current_move() {
            //     self.last_move = self.game.current_move();

            //     println!("RENDEINRGH EHE");

            //     // rerender
            // }

            self.render_game();


            // self.canvas.present();
            
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            // The rest of the game loop goes here...
        }
    }

    pub fn init_texturemap(&mut self, texture_map: &mut HashMap<String, Texture>) {
        let board = self.game.board();

        for y in (0..board.height()) {
            for x in 0..board.width() {
                let tile = board.tile_at(x, y).unwrap();
                let tile_rlock = tile.read().unwrap();
                let piece = tile_rlock.piece();

                if let Some(p) = piece {
                    let piece_rlock = p.read().unwrap();
                    
                    let path = format!("get texture from textures/{}/{}.png yes", piece_rlock.team_unchecked().name(), piece_rlock.name());
                    let key = format!("{}:{}", piece_rlock.team_unchecked().name(), piece_rlock.name());

                    let tex = self.texture_creator.load_texture(path).unwrap();



                    texture_map.insert(key, 
                        tex
                    );
                }
            }
        }
    }

    fn render_game(&mut self) {
        const TILE_WIDTH: u32 = 50;
        const TILE_HEIGHT: u32 = 50;
        let primary_color = Color::RGBA(0, 255, 0, 255);
        let secondary_color = Color::RGBA(255, 255, 255, 255);

        let board = self.game.board();
        let canvas = &mut self.canvas;
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for y in (0..board.height()) {
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
                canvas.fill_rect(Rect::new((x * TILE_WIDTH) as i32, (y * TILE_HEIGHT) as i32, TILE_WIDTH, TILE_HEIGHT)).unwrap();
                let tile = board.tile_at(x, y).unwrap();
                let tile_rlock = tile.read().unwrap();
                let piece = tile_rlock.piece();

                if let Some(p) = piece {
                    let piece_rlock = p.read().unwrap();
                    println!("get texture from textures/{}/{}.png yes", piece_rlock.team_unchecked().name(), piece_rlock.name());
                }
            }
        }

        canvas.present();
    }
}