use std::ops::Range;

use crate::ram::Ram;
use crate::cartprg::CartPrg;
use crate::ppu::Ppu;

pub struct BusCpu {
    ram: Ram,
    cartprg: CartPrg,
    ppu: Ppu
}

const ADDR_SPACE_RAM: Range<u16> = 0x0000..0x2000;
const ADDR_SPACE_PPU: Range<u16> = 0x2000..0x4000;
const ADDR_SPACE_APU_IO: Range<u16> = 0x4000..0x4020;
const ADDR_SPACE_CART_PRG: Range<u16> = 0x4020..0xffff;


impl BusCpu {

    pub fn new(ram: Ram, cartprg: CartPrg, ppu: Ppu) -> Self {
        return Self {
            ram, cartprg, ppu
        };
    }

    pub fn read(&self, addr: u16, read_only: bool) -> u8 {
        if ADDR_SPACE_RAM.contains(&addr) {
            print!("Reading from RAM");
            return self.ram.read(addr);
        } else if ADDR_SPACE_PPU.contains(&addr) {
            print!("Reading from PPU");
        } else if ADDR_SPACE_APU_IO.contains(&addr) {
            print!("Reading from APU/IO");
        } else if ADDR_SPACE_CART_PRG.contains(&addr) {
            print!("Reading from PROG ROM");
        }
        return 0x00;
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if ADDR_SPACE_RAM.contains(&addr) {
            print!("Writing to RAM");
            self.ram.write(addr, data);
        } else if ADDR_SPACE_PPU.contains(&addr) {
            print!("Writing to PPU");
        } else if ADDR_SPACE_APU_IO.contains(&addr) {
            print!("Writing to APU/IO");
        } else if ADDR_SPACE_CART_PRG.contains(&addr) {
            print!("Writing to PROG ROM");
        }
    }
}