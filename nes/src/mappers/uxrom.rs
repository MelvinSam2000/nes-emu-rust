use crate::cartridge::Cartridge;

use super::mapper::{Mapper, MapperOperations};

pub struct UxROM {
    banksel: u8,
}

impl UxROM {
    pub fn new() -> Self {
        Self { 
            banksel: 0 
        }
    }
}


impl MapperOperations for UxROM {

    fn read_prg(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        let mut mapped_addr  = 0u64;
        match addr {
            0x8000..=0xbfff => {
                if let Mapper::UxROM(uxrom) = &mut cart.mapper {
                    mapped_addr = (uxrom.banksel as u64)*0x4000 + (addr & 0x3fff) as u64;
                }
            },
            0xc000..=0xffff => {
                let prgbank_size: u16 = cart.prg_banks as u16 - 1;
                mapped_addr = (prgbank_size as u64)*0x4000  + (addr & 0x3fff) as u64;
            },
            _ => return Err(()),
        };
        Ok(cart.prgmem[mapped_addr as usize])
    }

    fn write_prg(cart: &mut Cartridge, addr: u16, data: u8) -> Result<(), ()> {
        match addr {
            0x8000..=0xffff => {
                if let Mapper::UxROM(uxrom) = &mut cart.mapper {
                    uxrom.banksel = data & 0x0f;
                    return Ok(());
                }
                Err(())
            },
            _ => Err(())
        }
    }

    fn read_chr(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        Ok(cart.chrmem[addr as usize])
    }

    fn write_chr(cart: &mut Cartridge, addr: u16, data: u8) -> Result<(), ()>{
        cart.chrmem[addr as usize] = data;
        Ok(())
    }

}