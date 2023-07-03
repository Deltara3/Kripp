pub mod cart;
pub mod c8;

pub struct Quirk {
    pub vf_reset: bool,
    pub memory: bool,
    pub display: bool,
    pub clipping: bool,
    pub shifting: bool,
    pub jumping: bool
}

impl Quirk {
    pub fn new(
        vf_reset: bool,
        memory: bool,
        display: bool,
        clipping: bool,
        shifting: bool,
        jumping: bool
    ) -> Quirk {
        return Quirk {
            vf_reset: vf_reset,
            memory: memory,
            display: display,
            clipping: clipping,
            shifting: shifting,
            jumping: jumping
        }
    }

    pub fn c8() -> Quirk {
        return Quirk::new(true, true, true, true, false, false);
    }

    pub fn load(&mut self) {
        // implement
    }

    pub fn save(&mut self) {
        // implement
    }
}
