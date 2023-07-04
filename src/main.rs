use krip::chip::{cart, c8, Quirk};
use krip::{SCR_W, SCR_H, CPF};
use std::time::{SystemTime, UNIX_EPOCH};
use eframe::egui;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let rom: Vec<u8> = cart::read(&args[1]);
    let mut chip: c8::CPU = c8::CPU::new(rom, Quirk::c8());

    let mut last = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let mut cycles = 8;

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(SCR_W as f32, SCR_H as f32)),
        ..Default::default()
    };

    eframe::run_simple_native("Kripp", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let paint = ui.painter();

            for _ in 0..cycles {
                chip.set_buttons(ui);
                chip.cycle();
            }

            chip.draw(paint);
            chip.decrement_timers();
            
            if chip.quirk.display {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                cycles = (now - last) * 500 / 1000;
                last = now;
            }

            ctx.request_repaint();
        });
    });
}

