use crate::chip::c8;
use macroquad::input::{KeyCode, is_key_down};

impl c8::CPU {
    pub fn set_buttons(&mut self) {
        self.keypad[0x01] = is_key_down(KeyCode::Key1);
        self.keypad[0x02] = is_key_down(KeyCode::Key2);
        self.keypad[0x03] = is_key_down(KeyCode::Key3);
        self.keypad[0x0C] = is_key_down(KeyCode::Key4);
        self.keypad[0x04] = is_key_down(KeyCode::Q);
        self.keypad[0x05] = is_key_down(KeyCode::W);
        self.keypad[0x06] = is_key_down(KeyCode::E);
        self.keypad[0x0D] = is_key_down(KeyCode::R);
        self.keypad[0x07] = is_key_down(KeyCode::A);
        self.keypad[0x08] = is_key_down(KeyCode::S);
        self.keypad[0x09] = is_key_down(KeyCode::D);
        self.keypad[0x0E] = is_key_down(KeyCode::F);
        self.keypad[0x0A] = is_key_down(KeyCode::Z);
        self.keypad[0x00] = is_key_down(KeyCode::X);
        self.keypad[0x0B] = is_key_down(KeyCode::C);
        self.keypad[0x0F] = is_key_down(KeyCode::V);
    }
}
