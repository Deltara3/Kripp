use crate::chip::c8;
use crate::{C8_W, C8_H};
use rand::Rng;

impl c8::CPU {
    /* CLS */
    pub fn c8_00e0(&mut self) {
        for y in 0..C8_H {
            for x in 0..C8_W {
                self.vram[y][x] = 0;
            }
        }
    }

    /* RET */
    pub fn c8_00ee(&mut self) {
        self.sp -= 1;
        self.pc = (self.stack[self.sp as usize] - 2) as usize;
    }
    
    /* JP addr */
    pub fn c8_1nnn(&mut self, nnn: usize) {
        self.pc = nnn - 2;
    }

    /* CALL addr */
    pub fn c8_2nnn(&mut self, nnn: usize) {
        self.stack[self.sp as usize] = (self.pc + 2) as u16;
        self.sp += 1;
        self.pc = nnn - 2;
    }

    /* SE Vx, byte */
    pub fn c8_3xnn(&mut self, x: usize, nn: u8) {
        self.pc += if self.v[x] == nn { 2 } else { 0 };
    }

    /* SNE Vx, byte */
    pub fn c8_4xnn(&mut self, x: usize, nn: u8) {
        self.pc += if self.v[x] != nn { 2 } else { 0 };
    }

    /* SE Vx, Vy */
    pub fn c8_5xy0(&mut self, x: usize, y: usize) {
        self.pc += if self.v[x] == self.v[y] { 2 } else { 0 };
    }

    /* LD Vx, byte */
    pub fn c8_6xnn(&mut self, x: usize, nn: u8) {
        self.v[x] = nn;
    }

    /* ADD Vx, byte */
    pub fn c8_7xnn(&mut self, x: usize, nn: u8) {
        self.v[x] = self.v[x].wrapping_add(nn);
    }

    /* LD Vx, Vy */
    pub fn c8_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    /* OR Vx, Vy */
    pub fn c8_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    /* AND Vx, Vy */
    pub fn c8_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    /* XOR Vx, Vy */
    pub fn c8_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    /* ADD Vx, Vy */
    pub fn c8_8xy4(&mut self, x: usize, y: usize) {
        let vx = self.v[x] as u16;
        let vy = self.v[y] as u16;
        let res = vx + vy;
        self.v[x] = res as u8;
        self.v[0x0F] = if res > 0xFF { 1 } else { 0 };
    }

    /* SUB Vx, Vy */
    pub fn c8_8xy5(&mut self, x: usize, y: usize) {
        self.v[0x0F] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    }

    /* SHR Vx {, Vy} */
    pub fn c8_8xy6(&mut self, x: usize, _y: usize) {
        self.v[0x0F] = self.v[x] & 1;
        self.v[x] >>= 1;
    }

    /* SUBN Vx, Vy */
    pub fn c8_8xy7(&mut self, x: usize, y: usize) {
        self.v[0x0F] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    }

    /* SHL Vx {, Vy} */
    pub fn c8_8xye(&mut self, x: usize, _y: usize) {
        self.v[0x0F] = (self.v[x] & 0b10000000) >> 7;
        self.v[x] <<= 1;
    }

    /* SNE Vx, Vy */
    pub fn c8_9xy0(&mut self, x: usize, y: usize) {
        self.pc += if self.v[x] != self.v[y] { 2 } else { 0 };
    }

    /* LD I, addr */
    pub fn c8_annn(&mut self, nnn: usize) {
        self.i = nnn;
    }

    /* JP V0, addr */
    pub fn c8_bnnn(&mut self, nnn: usize) {
        self.pc = (nnn + self.v[0] as usize) - 2;
    }

    /* RND Vx, byte */
    pub fn c8_cxnn(&mut self, x: usize, nn: u8) {
        let mut rng = rand::thread_rng();
        self.v[x] = rng.gen::<u8>() & nn;
    }

    /* DRW Vx, Vy, nibble */
    pub fn c8_dxyn(&mut self, x: usize, y: usize, n: usize) {
        self.v[0x0F] = 0;
        for byte in 0..n {
            let y = (self.v[y] as usize + byte) % C8_H;
            for bit in 0..8 {
                let x = (self.v[x] as usize + bit) % C8_W;
                let color = (self.ram[self.i + byte] >> (7 - bit)) & 1;
                self.v[0x0F] |= color & self.vram[y][x];
                self.vram[y][x] ^= color;
            }
        }
    }

    /* SKP Vx */
    pub fn c8_ex9e(&mut self, x: usize) {
        self.pc += if self.keypad[self.v[x] as usize] { 2 } else { 0 };
    }

    /* SKNP Vx */
    pub fn c8_exa1(&mut self, x: usize) {
        self.pc += if !self.keypad[self.v[x] as usize] { 2 } else { 0 };
    }

    /* LD Vx, K */
    pub fn c8_fx07(&mut self, x: usize) {
        self.v[x] = self.dt;
    }

    /* LD Vx, K */
    pub fn c8_fx0a(&mut self, x: usize) {
        self.waiting = true;
        self.register = x;
    }

    /* LD DT, Vx */
    pub fn c8_fx15(&mut self, x: usize) {
        self.dt = self.v[x];
    }

    /* LD ST, Vx */
    pub fn c8_fx18(&mut self, x: usize) {
        self.st = self.v[x];
    }

    /* ADD I, Vx */
    pub fn c8_fx1e(&mut self, x: usize) {
        self.i += self.v[x] as usize;
        self.v[0x0F] = if self.i > 0x0F00 { 1 } else { 0 };
    }

    /* LD F, Vx */
    pub fn c8_fx29(&mut self, x: usize) {
        self.i = (self.v[x] as usize) * 5;
    }

    /* LD B, Vx */
    pub fn c8_fx33(&mut self, x: usize) {
        self.ram[self.i] = self.v[x] / 100;
        self.ram[self.i + 1] = (self.v[x] % 100) / 10;
        self.ram[self.i + 2] = self.v[x] % 10;
    }

    /* LD [I], Vx */
    pub fn c8_fx55(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.ram[self.i + i] = self.v[i];
        }
    }

    /* LD Vx, [I] */
    pub fn c8_fx65(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.v[i] = self.ram[self.i + i];
        }
    }

    /* Invalid */
    pub fn invalid(&mut self, opcode: u16) {
        println!("[FATAL] {:#06x} at {:#06x} is invalid or unimplmented", opcode, self.pc);
        self.halted = true;
    }
}
