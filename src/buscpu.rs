use std::ops::Range;

use crate::nes::Nes;

pub struct BusCpu {
    pub ram: [u8; 0x2000]
}

const ADDR_SPACE_RAM: Range<u16> = 0x0000..0x2000;
const ADDR_SPACE_PPU: Range<u16> = 0x2000..0x4000;
const ADDR_SPACE_APU_IO: Range<u16> = 0x4000..0x4020;
const ADDR_SPACE_CART_PRG: Range<u16> = 0x4020..0xffff;


impl BusCpu {
    
    pub fn new() -> Self {
        return Self {
            ram: [0; 0x2000]
        };
    }
}

pub fn read(nes: &mut Nes, addr: u16) -> u8 {

    if ADDR_SPACE_RAM.contains(&addr) {
        return nes.buscpu.ram[addr as usize];
    } else if ADDR_SPACE_PPU.contains(&addr) {
        print!("Reading from PPU");
    } else if ADDR_SPACE_APU_IO.contains(&addr) {
        print!("Reading from APU/IO");
    } else if ADDR_SPACE_CART_PRG.contains(&addr) {
        return nes.cartprg.read(addr);
    }
    return nes.buscpu.ram[addr as usize];
}

pub fn write(nes: &mut Nes, addr: u16, data: u8) {

    if ADDR_SPACE_RAM.contains(&addr) {
        nes.buscpu.ram[addr as usize] = data;
    } else if ADDR_SPACE_PPU.contains(&addr) {
        print!("Reading from PPU");
    } else if ADDR_SPACE_APU_IO.contains(&addr) {
        print!("Reading from APU/IO");
    } else if ADDR_SPACE_CART_PRG.contains(&addr) {
        nes.cartprg.write(addr, data);
    }
}
