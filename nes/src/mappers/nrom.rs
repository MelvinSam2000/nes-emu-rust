use crate::cartridge::Cartridge;
use crate::mappers::mapper::MapperOperations;

pub struct NROM;

impl MapperOperations for NROM {

    fn read_prg(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        let mut mapped_addr = 0;
        if 0x8000 <= addr {
            if cart.prg_banks == 2 {
                mapped_addr = addr & 0x7fff;
            }
            if cart.prg_banks == 1 {
                mapped_addr = addr & 0x3fff;
            }
        }
        Ok(cart.prgmem[mapped_addr as usize])
    }

    fn write_prg(cart: &mut Cartridge, addr: u16, data: u8) -> Result<(), ()> {
        let mut mapped_addr = 0;
        if 0x8000 <= addr {
            if cart.prg_banks == 2 {
                mapped_addr = addr & 0x7fff;
            }
            if cart.prg_banks == 1 {
                mapped_addr = addr & 0x3fff;
            }
        }
        cart.prgmem[mapped_addr as usize] = data;
        Ok(())
    }

    fn read_chr(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        Ok(cart.chrmem[addr as usize])
    }

    fn write_chr(cart: &mut Cartridge, addr: u16, data: u8) -> Result<(), ()> {
        cart.chrmem[addr as usize] = data;
        Ok(())
    }
}
