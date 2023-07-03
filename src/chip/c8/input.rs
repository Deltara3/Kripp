use crate::chip::c8;
use macroquad::input::{KeyCode, is_key_down};

impl c8::CPU {
    pub fn set_buttons(&mut self) {
        macro_rules! keypad {
            ($($code:literal = $key:ident),+) => {
                $( self.keypad[$code] = is_key_down(KeyCode::$key); )+
            }
        }

        keypad! {
            0x01 = Key1,
            0x02 = Key2,
            0x03 = Key3,
            0x0C = Key4,
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
