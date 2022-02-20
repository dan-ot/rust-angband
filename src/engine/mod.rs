use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

pub struct Engine {
    pub canvas: Canvas<Window>,
    pub events: EventPump,
}

impl Engine {
    pub fn new(context: &Sdl) -> Engine {
        let video_subsystem = context.video().unwrap();
        let mode = video_subsystem.current_display_mode(0).unwrap();
        let window = video_subsystem
            .window(
                "rust-angband",
                (mode.w as f32 * 0.8) as u32,
                (mode.h as f32 * 0.8) as u32,
            )
            .maximized()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = context.event_pump().unwrap();

        Engine {
            canvas,
            events: event_pump,
        }
    }

    pub fn run(&mut self) -> () {
        'running: loop {
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();

            // TODO: DRAW
            // We'll need two things for this: a Graphics kit and a Font kit
            // To render a screen, we'll need a buffer (term?) of graphics or font references
            //  Maybe more than one buffer, for having a menu or 'subscreen' overlaid on the
            //  main menu or play field
            // We'll use these buffers to power our blits from the Graphics or Font kits

            self.canvas.present();
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }
        }
    }
}
