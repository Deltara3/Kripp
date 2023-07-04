use crate::chip::c8;
use eframe::egui::{Key, Ui};

impl c8::CPU {
    pub fn set_buttons(&mut self, ui: &Ui) {
        macro_rules! keypad {
            ($($code:literal = $key:ident),+) => {
                $( self.keypad[$code] = ui.input(|i| i.key_down(Key::$key)); )+
            }
        }

        keypad! {
            0x01 = Num1,
            0x02 = Num2,
            0x03 = Num3,
            0x0C = Num4,
            0x04 = Q,
            0x05 = W,
            0x06 = E,
            0x0D = R,
            0x07 = A,
            0x08 = S,
            0x09 = D,
            0x0E = F,
            0x0A = Z,
            0x00 = X,
            0x0B = C,
            0x0F = V
        }
    }
}

