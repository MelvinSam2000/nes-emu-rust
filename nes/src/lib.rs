pub mod nes;
pub mod cpu {
    pub mod cpu;
    pub mod instructions;
    pub mod addressing;
    pub mod decode;
}
pub mod ppu {
    pub mod ppu;
    pub mod regcontrol;
    pub mod regmask;
    pub mod regstatus;
    pub mod regscroll;
}
pub mod apu {
    pub mod apu;
    pub mod pulsechannel;
    pub mod trianglechannel;
}
pub mod mappers {
    pub mod mapper;
    pub mod nrom;
    pub mod uxrom;
    pub mod cnrom;
    pub mod gxrom;
}
pub mod tests {
    pub mod cputest;
    pub mod pputest;
    pub mod nesbench;
}
pub mod cartridge;
pub mod buscpu;
pub mod busppu;
pub mod joypad;