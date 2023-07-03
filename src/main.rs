use macroquad::prelude::*;
use krip::chip::cart;
use krip::chip::c8;
use krip::{SCR_W, SCR_H, SCALE, CPF};

fn conf() -> Conf {
    return Conf {
        window_title: "Kripp".to_owned(),
        window_resizable: false,
        window_width: SCR_W as i32,
        window_height: SCR_H as i32,
        ..Default::default()
    }
}

fn color(value: u8) -> Color {
    if value == 0 {
        return BLACK;
    } else {
        return WHITE;
    }
}

#[macroquad::main(conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let rom: Vec<u8> = cart::read(&args[1]);
    let mut chip: c8::CPU = c8::CPU::new(rom);

    loop {
        for _ in 0..CPF {
            chip.set_buttons();
            chip.cycle();
        }

        for (y, row) in chip.vram.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE;
                let y = (y as u32) * SCALE;
                draw_rectangle(x as f32, y as f32, SCALE as f32, SCALE as f32, color(col));
            }
        }
        
        next_frame().await
    }
}

