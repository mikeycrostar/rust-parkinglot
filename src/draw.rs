use rust_parkinglog::Colorable;
use sdl2::{pixels::Color, render::Canvas, video::Window};

use crate::{person::Person, transport::Transport};

pub struct Draw {
    pub canvas: Canvas<Window>,
}

impl Draw {
    pub fn new(canvas: Canvas<Window>) -> Draw {
        Draw { canvas }
    }

    pub fn transport(&mut self, transport: &Transport, position: usize) -> Result<(), String> {
        let position_x = position as i32 * 100;

        if transport.is_selected() {
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            self.canvas
                .fill_rect(sdl2::rect::Rect::new(position_x, 300, 20, 30))
                .unwrap();
        }

        let rect = sdl2::rect::Rect::new(position_x, 300, 20, 20);
        self.canvas.set_draw_color(transport.get_rgb());
        self.canvas.fill_rect(rect).unwrap();

        Ok(())
    }

    pub fn parking_lot(&mut self) -> Result<(), String> {
        let rect = sdl2::rect::Rect::new(30, 200, 300, 300);
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.fill_rect(rect).unwrap();

        Ok(())
    }

    pub fn person(&mut self, person: &Person, position: usize) -> Result<(), String> {
        let rect = sdl2::rect::Rect::new(position as i32 * 60 + 30, 60, 10, 10);
        self.canvas.set_draw_color(person.get_rgb());
        self.canvas.fill_rect(rect).unwrap();

        Ok(())
    }

    pub fn quay(&mut self, transport: Option<&Transport>, position: usize) -> Result<(), String> {
        let rect = sdl2::rect::Rect::new(position as i32 * 30 + 30, 200, 20, 20);

        match transport {
            Some(transport) => {
                self.canvas.set_draw_color(transport.get_rgb());
            }
            None => {
                self.canvas.set_draw_color(Color::RGB(120, 120, 120));
            }
        };

        self.canvas.fill_rect(rect).unwrap();

        Ok(())
    }
}
