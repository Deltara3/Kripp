use kripp::chip::{c8, Quirk};

fn main() {
    let mut chip: c8::CPU = c8::CPU::new(Quirk::c8());

    kripp::ui::start(chip);
}

