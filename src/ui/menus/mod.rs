use eframe::egui;
use crate::chip::{c8, cart};
use rfd::FileDialog;

pub fn bar(ui: &mut egui::Ui, chip: &mut c8::CPU) {
    egui::menu::bar(ui, |ui| {
        ui.menu_button("Interpreter", |ui| {
            if ui.button("Load").clicked() {
                let load = FileDialog::new()
                    .add_filter("CHIP-8", &["ch8", "c8"])
                    .pick_file();
                if load.is_some() {
                    chip.reset();
                    let location = load.unwrap();
                    chip.load(cart::read(location));
                    chip.running = true;
                } else { }
            }

            if chip.running {
                match chip.halted {
                    true => {
                        if ui.button("Continue").clicked() {
                            chip.halted = false;
                        }
                    },
                    _ => {
                        if ui.button("Pause").clicked() {
                            chip.halted = true;
                        }
                    }
                }
            } else {
                ui.add_enabled(false, egui::Button::new("Pause"));
            }
            
            match chip.running { 
                true => { 
                    if ui.button("Stop").clicked() {
                        chip.running = false;
                        chip.close();
                    }
                },
                _ => {
                    ui.add_enabled(false, egui::Button::new("Stop"));
                }
            }
        });

        ui.menu_button("Config", |ui| {

        });

        ui.menu_button("Tools", |ui| {

        });

        if ui.button("About").clicked() {
        
        }
    });
}
