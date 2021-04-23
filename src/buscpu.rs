use crate::nes::Nes;
use crate::ppu::ppu;

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
        0x4000 ..= 0x401f => {
            return 0;
        },
        0x4020 ..= 0xffff => {
            return nes.cartridge.prg_read(addr);
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
        0x4000 ..= 0x401f => {
        },
        0x4020 ..= 0xffff => {
            nes.cartridge.prg_write(addr, data);
        }
    }
}
