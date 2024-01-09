use chess::game::Game;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;
use std::time::Duration;

pub struct App {
    game: Game,
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
}

impl App {
    pub fn new(game: Game) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let event_pump = sdl_context.event_pump()?;
        
        Ok(App {
            game,
            canvas,
            event_pump,
        })
    }

    pub fn run(&mut self) {
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

            self.canvas.clear();
            self.canvas.present();
            
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            // The rest of the game loop goes here...
        }
    }
}