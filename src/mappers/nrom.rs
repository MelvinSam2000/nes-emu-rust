use crate::mappers::mapper::Mapper;
use crate::cartridge::Cartridge;

pub struct NRom;

impl Mapper for NRom {

    fn read_prg(&self, cart: &Cartridge, addr: u16) -> u16 {
        if 0x8000 <= addr {
            if cart.prg_banks == 2 {
               return addr - 0x8000;
            }
            if cart.prg_banks == 1 {
                return addr & 0x3fff;
            }
        }
        return addr;
    }

    fn write_prg(&self, cart: &Cartridge, addr: u16) -> u16 {
        if 0x8000 <= addr {
            if cart.prg_banks == 2 {
               return addr - 0x8000;
            }
            if cart.prg_banks == 1 {
                return addr & 0x3fff;
            }
        }
        return addr;
    }

    fn read_chr(&self, cart: &Cartridge, addr: u16) -> u16 {
        return addr;
    }

    fn write_chr(&self, cart: &Cartridge, addr: u16) -> u16 {
        return addr;
    }

}