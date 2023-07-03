use crate::chip::c8;
use crate::{SCALE, BACK, FORE};
use macroquad::prelude::*;

fn color(value: u8) -> Color {
    if value == 0 {
        return BACK;
    } else {
        return FORE;
    }
}

impl c8::CPU {
    pub fn draw(&self) {
        for (y, row) in self.vram.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE;
                let y = (y as u32) * SCALE;
                draw_rectangle(x as f32, y as f32, SCALE as f32, SCALE as f32, color(col));
            }
        }
    }
}
