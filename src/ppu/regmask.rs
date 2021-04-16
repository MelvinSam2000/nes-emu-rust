pub struct RegMask {
    pub reg: u8
}

enum Flag {
    Gr = 1 << 0,    // Greyscale
    m = 1 << 1,     // Background left column enable
    M = 1 << 2,     // Sprite left column enable
    b = 1 << 3,     // Background enable
    s = 1 << 4,     // Sprite enable
    R = 1 << 5,     // Color emphasis
    G = 1 << 6,     // Color emphasis
    B = 1 << 7,     // Color emphasis
}

impl RegMask {

    pub fn new() -> Self {
        return Self {
            reg: 0x00
        };
    }

    fn set_flag(&mut self, flag: Flag, val: bool) {
        if val {
            self.reg |= flag as u8;
        } else {
            self.reg &= !(flag as u8);
        }
    }

    fn get_flag(&self, flag: Flag) -> bool {
        return flag as u8 & self.reg != 0x00;
    }
}



