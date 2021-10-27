use crate::cartridge::Cartridge;

use super::mapper::MapperOperations;

pub struct GxROM;

impl MapperOperations for GxROM {
    fn read_prg(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        match addr {
            0x8000..=0xffff => {
                let mapped_addr = (cart.gxrom_banksel.0 as u16)*0x8000 + (addr & 0x7fff);
                Ok(cart.prgmem[mapped_addr as usize])
            },
            _ => Err(())
        }
    }

    fn write_prg(cart: &mut Cartridge, addr: u16, data: u8) -> Result<(), ()> {
        match addr {
            0x8000..=0xffff => {
                cart.gxrom_banksel = (data & 0x03, (data & 0x30) >> 4);
                Ok(())
            },
            _ => Err(())
        }
    }

    fn read_chr(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        match addr {
            0x0000..=0x1fff => {
                let mapped_addr = (cart.gxrom_banksel.1 as u16)*0x2000 + addr;
                Ok(cart.chrmem[mapped_addr as usize])
            },
            _ => Err(())
        }
    }

    fn write_chr(_: &mut Cartridge, _: u16, _: u8) -> Result<(), ()> {
        Err(())
    }
}
