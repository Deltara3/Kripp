pub mod cart;
pub mod c8;
pub mod disassembler;

pub struct Quirk {
    pub vf_reset: bool,
    pub memory: bool,
    pub display: bool,
    pub clipping: bool,
    pub shifting: bool,
    pub jumping: bool
}

impl Quirk {
    pub fn new() -> Quirk {
        return Quirk {
            vf_reset: false,
            memory: false,
            display: false,
            clipping: false,
            shifting: false,
            jumping: false
        }
    }

    pub fn c8(&mut self) {
        self.vf_reset = true;
        self.memory = true;
        self.display = true;
        self.clipping = true;
        self.shifting = false;
        self.jumping = false;
    }

    pub fn sc(&mut self) {
        self.vf_reset = false;
        self.memory = false;
        self.display = false;
        self.clipping = true;
        self.shifting = true;
        self.jumping = true;
    }

    pub fn xo(&mut self) {
        self.vf_reset = false;
        self.memory = true;
        self.display = false;
        self.clipping = false;
        self.shifting = false;
        self.jumping = false;
    }

    pub fn load(&mut self) {
        // implement
    }

    pub fn save(&mut self) {
        // implement
    }
}
