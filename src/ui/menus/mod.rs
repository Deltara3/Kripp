use eframe::egui;
use crate::chip::{c8, cart};
use rfd::FileDialog;

pub fn bar(ui: &mut egui::Ui, chip: &mut c8::CPU) {
    egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Load").clicked() {
                let load = FileDialog::new()
                    .add_filter("CHIP-8", &["ch8", "c8"])
                    .pick_file();
                if load.is_some() {
                    let location = load.unwrap();
                    println!("[LOAD] {}", location.display());
                    chip.load(cart::read(location));
                    chip.running = true;
                } else {
                    println!("[INFO] User canceled loading")
                }
            }

            if ui.button("Close").clicked() {
                if chip.running {
                    chip.close();
                    chip.running = false;
                }
            }
        });
    });
}
