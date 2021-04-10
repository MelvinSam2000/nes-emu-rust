use std::ops::Range;

use crate::cartprg::CartPrg;
use crate::ppu::Ppu;
use crate::mem::mem::Mem;

pub struct BusCpu {
    pub ram: [u8; 0x2000],
    pub cartprg: CartPrg,
    pub ppu: Ppu
}

const ADDR_SPACE_RAM: Range<u16> = 0x0000..0x2000;
const ADDR_SPACE_PPU: Range<u16> = 0x2000..0x4000;
const ADDR_SPACE_APU_IO: Range<u16> = 0x4000..0x4020;
const ADDR_SPACE_CART_PRG: Range<u16> = 0x4020..0xffff;


impl BusCpu {
    
    pub fn new(cartprg: CartPrg, ppu: Ppu) -> Self {
        return Self {
            ram: [0; 0x2000], cartprg, ppu
        };
    }
}

impl Mem for BusCpu {

    fn read(&self, addr: u16) -> u8 {

        if ADDR_SPACE_RAM.contains(&addr) {
            return self.ram[addr as usize];
        } else if ADDR_SPACE_PPU.contains(&addr) {
            print!("Reading from PPU");
        } else if ADDR_SPACE_APU_IO.contains(&addr) {
            print!("Reading from APU/IO");
        } else if ADDR_SPACE_CART_PRG.contains(&addr) {
            return self.cartprg.read(addr);
        }
        return self.ram[addr as usize];
    }

    fn write(&mut self, addr: u16, data: u8) {
        if ADDR_SPACE_RAM.contains(&addr) {
            self.ram[addr as usize] = data;
        } else if ADDR_SPACE_PPU.contains(&addr) {
            print!("Reading from PPU");
        } else if ADDR_SPACE_APU_IO.contains(&addr) {
            print!("Reading from APU/IO");
        } else if ADDR_SPACE_CART_PRG.contains(&addr) {
            self.cartprg.write(addr, data);
        }
    }
}