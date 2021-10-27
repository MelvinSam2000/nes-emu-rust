use crate::cartridge::Cartridge;

use super::mapper::{Mapper, MapperOperations};

pub struct GxROM {
    banksel: (u8, u8)
}

impl GxROM {
    pub fn new() -> Self {
        Self { 
            banksel: (0, 0)
        }
    }
}


impl MapperOperations for GxROM {
    fn read_prg(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        match addr {
            0x8000..=0xffff => {
                if let Mapper::GxROM(gxrom) = &cart.mapper {
                    let mapped_addr = (gxrom.banksel.0 as u16)*0x8000 + (addr & 0x7fff);
                    return Ok(cart.prgmem[mapped_addr as usize]);
                }
                Err(())
            },
            _ => Err(())
        }
    }

    fn write_prg(cart: &mut Cartridge, addr: u16, data: u8) -> Result<(), ()> {
        match addr {
            0x8000..=0xffff => {
                if let Mapper::GxROM(gxrom) = &mut cart.mapper {
                    gxrom.banksel = (data & 0x03, (data & 0x30) >> 4);
                    return Ok(());
                }
                Err(())
            },
            _ => Err(())
        }
    }

    fn read_chr(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        match addr {
            0x0000..=0x1fff => {
                if let Mapper::GxROM(gxrom) = &cart.mapper {
                    let mapped_addr = (gxrom.banksel.1 as u16)*0x2000 + addr;
                    return Ok(cart.chrmem[mapped_addr as usize]);
                }
                Err(())
            },
            _ => Err(())
        }
    }

    fn write_chr(_: &mut Cartridge, _: u16, _: u8) -> Result<(), ()> {
        Err(())
    }
}
