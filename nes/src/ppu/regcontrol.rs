#![allow(non_upper_case_globals)]

use bitflags::bitflags;

bitflags! {
    pub struct RegControl: u8 {
        const Nx = 1 << 0;   // Nametable select (x)
        const Ny = 1 << 1;   // Nametable select (y)
        const I = 1 << 2;     // Increment mode
        const S = 1 << 3;     // Sprite tile select
        const B = 1 << 4;     // Background tile select
        const H = 1 << 5;     // Sprite height
        const P = 1 << 6;     // PPU master/slave
        const V = 1 << 7;     // NMI enable
    }
}

impl RegControl {

    pub fn new() -> Self {
        RegControl::from_bits_truncate(0)
    }

    pub fn update(&mut self, data: u8) {
        self.bits = data;
    } 

    pub fn is_nmi_enabled(&mut self) -> bool {
        self.contains(RegControl::V)
    }

    pub fn is_inc_mode(&mut self) -> bool {
        self.contains(RegControl::I)
    }

    pub fn get_bg(&self) -> bool {
        self.contains(RegControl::B)
    }

    /*
    
    pub fn set_nmi_enabled(&mut self, value: bool) {
        self.set_flag(Flag::V, value);
    }

    pub fn get_name_x(&self) -> bool {
        return self.get_flag(Flag::Nx);
    }

    pub fn get_name_y(&self) -> bool {
        return self.get_flag(Flag::Ny);
    }

    pub fn set_name_x(&mut self, val: bool) {
        self.set_flag(Flag::Nx, val);
    }

    pub fn set_name_y(&mut self, val: bool) {
        self.set_flag(Flag::Ny, val);
    }

    pub fn get_bg(&self) -> bool {
        return self.get_flag(Flag::B);
    }

    pub fn set_bg(&mut self, val: bool) {
        self.set_flag(Flag::B, val);
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
    */
}



