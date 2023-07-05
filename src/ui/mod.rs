use crate::{SCR_W, SCR_H, CPF};
use crate::chip::c8;
use std::time::{SystemTime, UNIX_EPOCH};
use eframe::egui;
use eframe::egui::menu;
mod menus;

pub fn start(mut chip: c8::CPU) {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(SCR_W as f32, SCR_H as f32)),
        resizable: false,
        ..Default::default()
    };

    let mut last = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let mut cycles = 8;

    eframe::run_simple_native("Kripp", options, move |ctx, frame| {
        egui::TopBottomPanel::top("menu_bar").exact_height(20.0).show(ctx, |ui| {
            menus::bar(ui, &mut chip);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let paint = ui.painter();

            if chip.running {
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
            }

            ctx.request_repaint();
        });
    });
}
