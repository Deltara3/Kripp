use macroquad::prelude::*;
use krip::chip::{cart, c8, Quirk};
use krip::{SCR_W, SCR_H, CPF};
use std::time::{SystemTime, UNIX_EPOCH};

fn conf() -> Conf {
    return Conf {
        window_title: "Kripp".to_owned(),
        window_resizable: false,
        window_width: SCR_W as i32,
        window_height: SCR_H as i32,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let rom: Vec<u8> = cart::read(&args[1]);
    let mut chip: c8::CPU = c8::CPU::new(rom, Quirk::c8());

    let mut last = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let mut cycles = 8;

    loop {
        for _ in 0..cycles {
            chip.set_buttons();
            chip.cycle();
        }
        
        chip.draw();
        chip.decrement_timers();
   
        if chip.quirk.display {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            cycles = (now - last) * 500 / 1000;
            last = now;
        }
        
        next_frame().await
    }
}

