use crate::chip::c8;
use eframe::egui::{Painter, Rounding, Pos2, Color32, Rect, Vec2};

fn color(value: u8, colors: [Color32; 2]) -> Color32 {
    if value == 0 {
        return colors[0];
    } else {
        return colors[1];
    }
}

impl c8::CPU {
    pub fn draw(&self, draw: &Painter, size: Vec2, colors: [Color32; 2]) {
        let scale;
        let scale_x = size[0] / 64.0;
        let scale_y = (size[1] - 24.0) / 32.0;

        if scale_x <= scale_y {
            scale = scale_x as u32;
        } else {
            scale = scale_y as u32;
        }

        for (y, row) in self.vram.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32 * scale) as u32;
                let y = (y as u32 * scale) as u32 + 24;
                let offset_x = (size[0] as u32 / 2) - (32 * scale);
                let offset_y = (((size[1] as u32 - 24) / 2) + 24) - ((16 * scale) + 24);


                draw.rect_filled(
                    Rect::from_min_max(
                        Pos2::new(x as f32 + offset_x as f32, y as f32 + offset_y as f32),
                        Pos2::new(x as f32 + scale as f32 + offset_x as f32, y as f32 + scale as f32 + offset_y as f32),

                    ),
                    Rounding::none(),
                    color(col, colors)
                )
            }
        }
    }
}
