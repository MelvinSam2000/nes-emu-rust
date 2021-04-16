pub struct RegControl {
    pub reg: u8
}

enum Flag {
    Nlo = 1 << 0,   // Nametable select (low bit)
    Nhi = 1 << 1,   // Nametable select (high bit)
    I = 1 << 2,     // Increment mode
    S = 1 << 3,     // Sprite tile select
    B = 1 << 4,     // Background tile select
    H = 1 << 5,     // Sprite height
    P = 1 << 6,     // PPU master/slave
    V = 1 << 7,     // NMI enable
}

impl RegControl {

    pub fn new() -> Self {
        return Self {
            reg: 0x00
        };
    }

    pub fn is_nmi_enabled(&mut self) -> bool {
        return self.get_flag(Flag::V);
    }
    
    pub fn set_nmi_enabled(&mut self, value: bool) {
        self.set_flag(Flag::V, value);
    }

    pub fn is_inc_mode(&mut self) -> bool {
        return self.get_flag(Flag::I);
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



