use crate::chip::c8;
use crate::{SCALE};
use eframe::egui::{Painter, Rounding, Pos2, Color32, Rect};

fn color(value: u8) -> Color32 {
    if value == 0 {
        return Color32::from_rgb(0, 0, 0);
    } else {
        return Color32::from_rgb(255, 255, 255);
    }
}

impl c8::CPU {
    pub fn draw(&self, draw: &Painter) {
        for (y, row) in self.vram.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE;
                let y = ((y as u32) * SCALE) + 24;
                draw.rect_filled(
                    Rect::from_min_max(
                        Pos2::new(x as f32, y as f32),
                        Pos2::new(x as f32 + SCALE as f32, y as f32 + SCALE as f32),
                    ),
                    Rounding::none(),
                    color(col)
                )
            }
        }
    }
}
