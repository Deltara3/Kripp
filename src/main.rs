mod ui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(960.0, 504.0)), /* 960x480 (including menu bar) */
        min_window_size: Some(eframe::egui::vec2(64.0, 56.0)),
        ..Default::default()
    };
    
    return eframe::run_native(
        "Kripp",
        options,
        Box::new(|_cc| Box::<ui::Kripp>::default())
    );
}

