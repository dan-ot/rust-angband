use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, CanvasBuilder};
use sdl2::{EventPump, Sdl};
use sdl2::ttf::Sdl2TtfContext;

use crate::ui::FontAtlas;
use crate::ui::graphics::GraphicsModeService;

pub struct Engine<'a> {
    pub canvas: Box<WindowCanvas>,
    pub events: EventPump,
    pub atlas: Box<FontAtlas<'a>>,
    pub graphics_modes: GraphicsModeService
}

impl Engine<'_> {
    pub fn new(context: &Sdl, mut canvas: Box<WindowCanvas>, font_context: Sdl2TtfContext, graphics: GraphicsModeService) -> Engine {
        let texture_creator = canvas.texture_creator();

        let atlas = Box::new(FontAtlas::render_atlas(&graphics.fonts[graphics.current_font], &font_context, &mut canvas, &texture_creator));

        let event_pump = context.event_pump().unwrap();

        Engine {
            canvas,
            events: event_pump,
            atlas,
            graphics_modes: graphics
        }
    }

    pub fn run(&mut self) {
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
