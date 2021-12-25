use sdl2::{Sdl, EventPump};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Engine {
    pub canvas: Canvas<Window>,
    pub events: EventPump
}

impl Engine {
    pub fn new(context: &Sdl) -> Engine {
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem.window("rust-angband", 800, 600)
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = context.event_pump().unwrap();

        Engine {
            canvas,
            events: event_pump
        }
    }

    pub fn run(&mut self) -> () {
        
        'running: loop {
            self.canvas.set_draw_color(Color::BLUE);
            self.canvas.clear();
            self.canvas.present();
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit {..} 
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    _ => { }
                }
            }
        }
    }
}