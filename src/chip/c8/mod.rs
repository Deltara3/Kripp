use crate::{C8_W, C8_H, C8_FONT};
use crate::chip::Quirk;
mod opcodes;
mod input;
mod display;

pub struct CPU {
    pub vram: [[u8; C8_W]; C8_H],
    ram: [u8; 4096],
    v: [u8; 16],
    stack: [u16; 16],
    i: usize,
    pc: usize,
    sp: u8,
    st: u8,
    dt: u8,
    pub halted: bool,
    keypad: [bool; 16],
    register: usize,
    pub quirk: Quirk,
    pub running: bool
}

impl CPU {
    pub fn new(quirks: Quirk) -> CPU {
        let mut ram = [0; 4096];
        for value in 0..C8_FONT.len() { ram[value] = C8_FONT[value]; }
  
        return CPU {
            vram: [[0; C8_W];  C8_H],
            ram: ram,
            v: [0; 16],
            stack: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            st: 0,
            dt: 0,
            halted: false,
            keypad: [false; 16],
            register: 0,
            quirk: quirks,
            running: false
        }
    }

    pub fn load(&mut self, rom: Vec<u8>) {
        let mut current_address: usize = 0x200;
        for byte in rom {
            if current_address >= 4096 {
                println!("[FATAL] Rom supplied was too large");
                std::process::exit(1);
            } else {
                self.ram[current_address] = byte;
            }
            current_address += 1;
        }
    }

    pub fn close(&mut self) {
        self.load(vec![0; 3584]);
    }

    fn get_opcode(&self) -> u16 {
        return (self.ram[self.pc] as u16) << 8 | self.ram[self.pc + 1] as u16;
    }

    pub fn cycle(&mut self) {
        if self.halted {
            for i in 0..self.keypad.len() {
                if self.keypad[i] {
                    // self.v[self.register] = i as u8;
                    self.halted = false; 
                    break;
                }
            }
        } else {
            let opcode: u16 = self.get_opcode();
            self.execute(opcode);
        }
    }

    pub fn decrement_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn execute(&mut self, opcode: u16) {
        let byte = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8
        );

        let nnn = (opcode & 0x0FFF) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let n = byte.3 as usize;
        let x = byte.1 as usize;
        let y = byte.2 as usize;

        if !self.halted {
            println!("[EXEC] {:#06x} at {:#06x}", opcode, self.pc);
            match byte {
                (0x00, 0x00, 0x0E, 0x00) => self.c8_00e0(),
                (0x00, 0x00, 0x0E, 0x0E) => self.c8_00ee(),
                (0x01, _, _, _) => self.c8_1nnn(nnn),
                (0x02, _, _, _) => self.c8_2nnn(nnn),
                (0x03, _, _, _) => self.c8_3xnn(x, nn),
                (0x04, _, _, _) => self.c8_4xnn(x, nn),
                (0x05, _, _, 0x00) => self.c8_5xy0(x, y),
                (0x06, _, _, _) => self.c8_6xnn(x, nn),
                (0x07, _, _, _) => self.c8_7xnn(x, nn),
                (0x08, _, _, 0x00) => self.c8_8xy0(x, y),
                (0x08, _, _, 0x01) => self.c8_8xy1(x, y),
                (0x08, _, _, 0x02) => self.c8_8xy2(x, y),
                (0x08, _, _, 0x03) => self.c8_8xy3(x, y),
                (0x08, _, _, 0x04) => self.c8_8xy4(x, y),
                (0x08, _, _, 0x05) => self.c8_8xy5(x, y),
                (0x08, _, _, 0x06) => self.c8_8xy6(x, y),
                (0x08, _, _, 0x07) => self.c8_8xy7(x, y),
                (0x08, _, _, 0x0E) => self.c8_8xye(x, y),
                (0x09, _, _, 0x00) => self.c8_9xy0(x, y),
                (0x0A, _, _, _) => self.c8_annn(nnn),
                (0x0B, _, _, _) => self.c8_bnnn(nnn),
                (0x0C, _, _, _) => self.c8_cxnn(x, nn),
                (0x0D, _, _, _) => self.c8_dxyn(x, y, n),
                (0x0E, _, 0x09, 0x0E) => self.c8_ex9e(x),
                (0x0E, _, 0x0A, 0x01) => self.c8_exa1(x),
                (0x0F, _, 0x00, 0x07) => self.c8_fx07(x),
                (0x0F, _, 0x00, 0x0A) => self.c8_fx0a(x),
                (0x0F, _, 0x01, 0x05) => self.c8_fx15(x),
                (0x0F, _, 0x01, 0x08) => self.c8_fx18(x),
                (0x0F, _, 0x01, 0x0E) => self.c8_fx1e(x),
                (0x0F, _, 0x02, 0x09) => self.c8_fx29(x),
                (0x0F, _, 0x03, 0x03) => self.c8_fx33(x),
                (0x0F, _, 0x05, 0x05) => self.c8_fx55(x),
                (0x0F, _, 0x06, 0x05) => self.c8_fx65(x),
                _ => self.invalid(opcode)
            }
        }
        self.pc += 2;
    }
}
