/* Worst file incoming */
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use kripp::chip::{c8, cart, Quirk, disassembler};
use std::time::{SystemTime, UNIX_EPOCH};
use rfd::FileDialog;

#[derive(PartialEq)]
enum QuirkPreset {
    C8,
    SC,
    XO
}

impl std::fmt::Display for QuirkPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            QuirkPreset::C8 => write!(f, "CHIP-8"),
            QuirkPreset::SC => write!(f, "SUPER-CHIP"),
            QuirkPreset::XO => write!(f, "XO-CHIP")
        }
    }
}

struct State {
    colors: [egui::Color32; 2],
    disassembly: Vec<[String; 3]>,
    about_opened: bool,
    input_opened: bool,
    disassembler_opened: bool,
    registers_opened: bool,
    quirk_opened: bool,
    quirk_preset: QuirkPreset,
    interpretation_opened: bool
}

impl State {
    pub fn new() -> State {
        return State {
            colors: [egui::Color32::BLACK, egui::Color32::WHITE],
            disassembly: vec![[String::from(""), String::from(""), String::from("")]],
            about_opened: false,
            input_opened: false,
            disassembler_opened: false,
            registers_opened: false,
            quirk_opened: false,
            quirk_preset: QuirkPreset::C8,
            interpretation_opened: false
        }
    }
}

pub struct Kripp {
    state: State,
    chip: c8::CPU,
    last: u128,
    cps: u128,
    fixed_cps: f32
}

impl Default for Kripp {
    fn default() -> Kripp {
        let mut quirks = Quirk::new();
        quirks.c8();
        return Kripp {
            state: State::new(),
            chip: c8::CPU::new(quirks),
            last: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
            cps: 8,
            fixed_cps: 8.0
        }
    }
}

impl eframe::App for Kripp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut visual = egui::Visuals::dark();
        visual.window_shadow = egui::epaint::Shadow::NONE;
        visual.popup_shadow = egui::epaint::Shadow::NONE;
        ctx.set_visuals(visual);

        egui::TopBottomPanel::top("Menu").exact_height(20.0).show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("🖳 Interpreter", |ui| {
                    if ui.button("🗁 Load").clicked() {
                        let load = FileDialog::new()
                            .add_filter("CHIP-8", &["ch8", "c8"])
                            .pick_file();
                        if load.is_some() {
                            self.chip.reset();
                            let location = load.unwrap();
                            let cart = cart::read(location);
                            self.chip.load(cart.clone());
                            self.chip.running = true;
                            self.chip.halted = false;
                            self.state.disassembly = disassembler::rom(cart);
                        } else {
                            /* User canceled */
                        }
                        ui.close_menu();
                    }

                    if self.chip.running {
                        match self.chip.halted {
                            true => {
                                if ui.button("▶ Continue").clicked() {
                                    self.chip.halted = false;
                                    ui.close_menu();
                                }
                            },
                            _ => {
                                if ui.button("⏸ Pause").clicked() {
                                    self.chip.halted = true;
                                    ui.close_menu();
                                }
                            }
                        }

                        if ui.button("⟲ Reset").clicked() {
                            self.chip.reset();
                            self.chip.halted = false;
                            ui.close_menu();
                        }
                    } else {
                        ui.add_enabled(false, egui::Button::new("⏸ Pause"));
                        ui.add_enabled(false, egui::Button::new("⟲ Reset"));
                    }

                    match self.chip.running {
                        true => {
                            if ui.button("🗙 Close").clicked() {
                                self.chip.close();
                                self.chip.running = false;
                                self.state.disassembly = vec![[String::from(""), String::from(""), String::from("")]];
                                ui.close_menu();
                            }
                        },
                        _ => {
                            ui.add_enabled(false, egui::Button::new("🗙 Close"));
                        }
                    }

                    ui.separator();

                    if ui.button("⎆ Quit").clicked() {
                        frame.close();
                        ui.close_menu();
                    }
                });

                ui.menu_button("⚙ Config", |ui| {
                    if ui.button("🕹 Input").clicked() {
                        self.state.input_opened = true;
                        ui.close_menu();
                    }
                    if ui.button("⚛ Quirk").clicked() {
                        self.state.quirk_opened = true;
                        ui.close_menu();
                    }
                    if ui.button("🖳 Interpretation").clicked() {
                        self.state.interpretation_opened = true;
                        ui.close_menu();
                    }
                });

                ui.menu_button("🔧 Tools", |ui| {
                    if ui.checkbox(&mut self.state.disassembler_opened, "📦 Disassembler").clicked() {
                        ui.close_menu();
                    }
                    if ui.checkbox(&mut self.state.registers_opened, "🔎 Registers").clicked() {
                        ui.close_menu();
                    }
                });

                if ui.button("ℹ About").clicked() {
                    self.state.about_opened = true;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let size = frame.info().window_info.size;
            let paint = ui.painter();

            if self.chip.running {
                let select = match self.chip.quirk.display {
                    true => self.cps,
                    _ => self.fixed_cps as u128
                };

                for _ in 0..select {
                    self.chip.set_buttons(ui);
                    self.chip.cycle();
                }

                self.chip.draw(paint, size, self.state.colors);
                self.chip.decrement_timers();

                if self.chip.quirk.display {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                    self.cps = (now - self.last) * 500 / 1000;
                    self.last = now;
                }
            }


            egui::Window::new("ℹ About")
                .open(&mut self.state.about_opened)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("Kripp is a CHIP-8 and derivatives interpreter");
                    ui.label("Made with ♡ by Deltara");
                    ui.separator();
                    ui.label(egui::RichText::new(format!("Version: {}", egui::special_emojis::GITHUB)).weak().size(10.0));
                    let platform_str = format!("Platform: {}", match ctx.os() {
                        egui::os::OperatingSystem::Windows => egui::special_emojis::OS_WINDOWS,
                        egui::os::OperatingSystem::Mac => egui::special_emojis::OS_APPLE,
                        egui::os::OperatingSystem::Nix => egui::special_emojis::OS_LINUX,
                        _ => '❓'
                    });
                    ui.label(egui::RichText::new(platform_str).weak().size(10.0));
                });

            /* TODO: Implement input mapping */ 
            egui::Window::new("🕹 Input")
                .open(&mut self.state.input_opened)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("To be implemented"); 
                });

            let dis = &self.state.disassembly;

            egui::Window::new("📦 Disassembler")
                .open(&mut self.state.disassembler_opened)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        if dis.len() != 0 {            
                            TableBuilder::new(ui)
                                .striped(true)
                                .column(Column::exact(10.0))
                                .column(Column::exact(50.0))
                                .column(Column::exact(45.0))
                                .column(Column::exact(120.0))
                                .header(22.0, |mut header| {
                                    header.col(|ui| {
                                        ui.heading(" ");
                                    });
                                    header.col(|ui| {
                                        ui.heading("At");
                                    });
                                    header.col(|ui| {
                                        ui.heading("Op");
                                    });
                                    header.col(|ui| {
                                        ui.heading("Mnemonic");
                                    });
                                })
                                .body(|body| {
                                    body.rows(20.0, dis.len(), |row_index, mut row| {
                                        row.col(|ui| {
                                            ui.label(match row_index == self.chip.pc - 0x200 {
                                                true => "➡",
                                                false => " "
                                            });
                                        });
                                        row.col(|ui| {
                                            ui.label(dis[row_index][0].clone());
                                        });
                                        row.col(|ui| {
                                            ui.label(dis[row_index][1].clone());
                                        });
                                        row.col(|ui| {
                                            ui.label(dis[row_index][2].clone());
                                        });
                                    });
                                });
                        } else {
                            ui.label("Nothing loaded");
                        }
                    });
                });
                

            egui::Window::new("🔎 Registers")
                .open(&mut self.state.registers_opened)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        for i in 0..4 {
                            ui.label(format!("V{:0X}", i));
                            let mut reg = format!("{:#04X}", self.chip.v[i]);
                            ui.add_enabled(false, egui::TextEdit::singleline(&mut reg).desired_width(35.0));
                        }
                    });
                    ui.horizontal(|ui| {
                        for i in 4..8 {
                            ui.label(format!("V{:0X}", i));
                            let mut reg = format!("{:#04X}", self.chip.v[i]);
                            ui.add_enabled(false, egui::TextEdit::singleline(&mut reg).desired_width(35.0));
                        }
                    });
                    ui.horizontal(|ui| {
                        for i in 8..12 {
                            ui.label(format!("V{:0X}", i));
                            let mut reg = format!("{:#04X}", self.chip.v[i]);
                            ui.add_enabled(false, egui::TextEdit::singleline(&mut reg).desired_width(35.0));
                        }
                    });
                    ui.horizontal(|ui| {
                        for i in 12..16 {
                            ui.label(format!("V{:0X}", i));
                            let mut reg = format!("{:#04X}", self.chip.v[i]);
                            ui.add_enabled(false, egui::TextEdit::singleline(&mut reg).desired_width(35.0));
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("SP");
                        let mut sp = format!("{:#04X}", self.chip.sp);
                        ui.add_enabled(false, egui::TextEdit::singleline(&mut sp).desired_width(35.0));
                        ui.label("PC");
                        let mut pc = format!("{:#06X}", self.chip.pc);
                        ui.add_enabled(false, egui::TextEdit::singleline(&mut pc).desired_width(49.0));
                        ui.label("I");
                        let mut i = format!("{:#06X}", self.chip.i);
                        ui.add_enabled(false, egui::TextEdit::singleline(&mut i).desired_width(49.0));

                    });
                    ui.horizontal(|ui| {
                        ui.label("DT");
                        let mut dt = format!("{:#04X}", self.chip.dt);
                        ui.add_enabled(false, egui::TextEdit::singleline(&mut dt).desired_width(35.0));
                        ui.label("ST");
                        let mut st = format!("{:#04X}", self.chip.st);
                        ui.add_enabled(false, egui::TextEdit::singleline(&mut st).desired_width(35.0));
                    });
                });

            egui::Window::new("⚛ Quirk")
                .open(&mut self.state.quirk_opened)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.checkbox(&mut self.chip.quirk.vf_reset, "VF Reset");
                    ui.checkbox(&mut self.chip.quirk.memory, "Memory");
                    ui.checkbox(&mut self.chip.quirk.display, "Display");
                    ui.checkbox(&mut self.chip.quirk.clipping, "Clipping");
                    ui.checkbox(&mut self.chip.quirk.shifting, "Shifting");
                    ui.checkbox(&mut self.chip.quirk.jumping, "Jumping");
                    ui.horizontal(|ui| {
                        ui.label("Preset");
                        egui::ComboBox::from_id_source("Preset")
                            .selected_text(format!("{}", self.state.quirk_preset))
                            .show_ui(ui, |ui| {
                                if ui.selectable_value(&mut self.state.quirk_preset, QuirkPreset::C8, "CHIP-8").clicked() ||
                                ui.selectable_value(&mut self.state.quirk_preset, QuirkPreset::SC, "SUPER-CHIP").clicked() ||
                                ui.selectable_value(&mut self.state.quirk_preset, QuirkPreset::XO, "XO-CHIP").clicked() {
                                    match self.state.quirk_preset {
                                        QuirkPreset::C8 => self.chip.quirk.c8(),
                                        QuirkPreset::SC => self.chip.quirk.sc(),
                                        QuirkPreset::XO => self.chip.quirk.xo()
                                    }
                                }
                            });
                    });
                });

            egui::Window::new("🖳 Interpretation")
                .open(&mut self.state.interpretation_opened)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Cycles");
                        ui.add(
                            egui::widgets::Slider::new(&mut self.fixed_cps, 1.0..=1000.0)
                                .fixed_decimals(0)
                                .trailing_fill(true)
                                .step_by(1.0)
                                .logarithmic(true)
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Background Color");
                            egui::widgets::color_picker::color_picker_color32(
                                ui,
                                &mut self.state.colors[0],
                                egui::widgets::color_picker::Alpha::Opaque
                            );
                        });
                        ui.vertical(|ui| {
                            ui.label("Foreground Color");
                            egui::widgets::color_picker::color_picker_color32(
                                ui,
                                &mut self.state.colors[1],
                                egui::widgets::color_picker::Alpha::Opaque
                            );
                        });
                    });
                });

            ctx.request_repaint();
        });
    }
}
