pub struct RegStatus {
    pub reg: u8
}

enum Flag {
    O = 1 << 5,     // sprite overflow
    S = 1 << 6,     // sprite 0 hit
    V = 1 << 7,     // vblank
}

impl RegStatus {

    pub fn new() -> Self {
        return Self {
            reg: 0x00
        };
    }

    pub fn set_vblank(&mut self, val: bool){
        self.set_flag(Flag::V, val);
    }

    pub fn get_vblank(&mut self) -> bool {
        return self.get_flag(Flag::V);
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



