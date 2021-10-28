use crate::nes::Nes;
use crate::cartridge::{self, Mirroring};

pub struct BusPpu {
    pub vram: [u8; 0x1000],
    pub palette: [u8; 0x20],
}

impl BusPpu {
    
    pub fn new() -> Self {
        return Self {
            vram: [0; 0x1000],
            palette: [0; 0x20]
        };
    }
}

pub fn read(nes: &mut Nes, addr: u16) -> u8 {

    match addr {
        0x0000 ..= 0x1fff => {
            cartridge::chr_read(nes, addr)
        },
        0x2000 ..= 0x2fff => {
            let mapped_addr = mirror_vram_addr(nes, addr);
            nes.busppu.vram[mapped_addr as usize]
        },
        0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
            let add_mirror = addr - 0x10;
            nes.busppu.palette[(add_mirror - 0x3f00) as usize]
        },
        0x3f00 ..= 0x3fff => {
            let addr_mirror = addr & 0x3f1f;
            nes.busppu.palette[(addr_mirror - 0x3f00) as usize]
        },
        _ => {
            0x00
        }
    }
}

pub fn write(nes: &mut Nes, addr: u16, data: u8) {

    match addr {
        0x0000 ..= 0x1fff => {
            cartridge::chr_write(nes, addr, data);
        },
        0x2000 ..= 0x2fff => {
            let mapped_addr = mirror_vram_addr(nes, addr);
            nes.busppu.vram[mapped_addr as usize] = data;
        },
        0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
            let add_mirror = addr - 0x10;
            nes.busppu.palette[(add_mirror - 0x3f00) as usize] = data;
        },
        0x3f00 ..= 0x3fff => {
            let addr_mirror = addr & 0x3f1f;
            nes.busppu.palette[(addr_mirror - 0x3f00) as usize] = data;
        },
        _ => {
        }
    }
}

pub fn mirror_vram_addr(nes: &mut Nes, addr: u16) -> u16 {
    let mut mapped_addr = addr & 0x0fff;
    match &nes.cartridge.mirroring {
        Mirroring::HORIZONTAL => {
            if mapped_addr >= 0x0400 && mapped_addr < 0x0800 {
                mapped_addr -= 0x0400;
            } else if mapped_addr >= 0x0c00 {
                mapped_addr -= 0x0400;
            }
        },
        Mirroring::VERTICAL => {
            if mapped_addr >= 0x0800 && mapped_addr < 0x0c00 {
                mapped_addr -= 0x0800;
            } else if mapped_addr >= 0x0c00 {
                mapped_addr -= 0x0800;
            }
        },
        _ => {},
    }
    mapped_addr
}
