use crate::cartridge;
use crate::nes::Nes;
use crate::ppu::ppu;
use crate::apu::apu;

pub struct BusCpu {
    pub ram: [u8; 0x0800]
}

impl BusCpu {
    
    pub fn new() -> Self {
        return Self {
            ram: [0; 0x0800]
        };
    }
}

pub fn read(nes: &mut Nes, addr: u16) -> u8 {

    match addr {
        0x0000 ..= 0x1fff => {
            return nes.buscpu.ram[addr as usize & 0x07ff];
        },
        0x2000 ..= 0x3fff => {
            return ppu::read_ppu_reg(nes, addr & 0x2007);
        },
        0x4016 => {
            return nes.joypad.read();
        },
        0x4000 ..= 0x4013 | 0x4015 => {
            return apu::read(nes, addr);
        },
        0x4020 ..= 0xffff => {
            return cartridge::prg_read(nes, addr);
        },
        _ => {
            return 0;
        }
    }
}

pub fn write(nes: &mut Nes, addr: u16, data: u8) {

    match addr {
        0x0000 ..= 0x1fff => {
            nes.buscpu.ram[addr as usize & 0x07ff] = data;
        },
        0x2000 ..= 0x3fff => {
            ppu::write_ppu_reg(nes, addr & 0x2007, data);
        },
        0x4014 => { // OAM DMA
            ppu::write_ppu_reg(nes, 0x4014, data);
        }
        0x4016 => {
            nes.joypad.write(data);
        },
        0x4000 ..= 0x4013 | 0x4015 => {
            apu::write(nes, addr, data);
        },
        0x4020 ..= 0xffff => {
            cartridge::prg_write(nes, addr, data);
        },
        _ => {}
    }
}
