mod nes;
mod cpu {
    pub mod cpu;
    pub mod instructions;
    pub mod addressing;
    pub mod decode;
}
mod ppu {
    pub mod ppu;
    pub mod regcontrol;
    pub mod regmask;
    pub mod regstatus;
}
mod mappers {
    pub mod mapper;
    pub mod nrom;
}
mod tests {
    pub mod cputest;
    pub mod pputest;
    pub mod nesbench;
}
mod events {
    pub mod drawevent;
}
mod cartridge;
mod buscpu;
mod busppu;
mod joypad;
mod wasmnes;