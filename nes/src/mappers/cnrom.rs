use crate::cartridge::Cartridge;

use super::mapper::{Mapper, MapperOperations};

pub struct CNROM {
    banksel: u8,
}

impl CNROM {
    pub fn new() -> Self {
        Self { 
            banksel: 0 
        }
    }
}


impl MapperOperations for CNROM {
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
        match addr {
            0x8000..=0xffff => {
                if let Mapper::CNROM(cnrom ) = &mut cart.mapper {
                    cnrom.banksel = (data & 0x03) as u8;
                }
                Ok(())
            },
            _ => Err(())
        }
    }

    fn read_chr(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        match addr {
            0x0000..=0x1fff => {
                if let Mapper::CNROM(cnrom ) = &cart.mapper {
                    let mapped_addr = (cnrom.banksel as u16)*0x2000 + addr;
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
