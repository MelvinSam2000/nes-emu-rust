use crate::cartridge::Cartridge;
use crate::mappers::nrom::NROM;
use crate::mappers::mmc1::MMC1;
use super::cnrom::CNROM;
use super::gxrom::GxROM;
use super::uxrom::UxROM;

pub trait MapperOperations {
    fn read_prg(cart: &mut Cartridge, addr: u16) -> Result<u8, ()>;
    fn write_prg(cart: &mut Cartridge, addr: u16, data: u8) -> Result<(), ()>;
    fn read_chr(cart: &mut Cartridge, addr: u16) -> Result<u8, ()>;
    fn write_chr(cart: &mut Cartridge, addr: u16, data :u8) -> Result<(), ()>;
}

pub enum Mapper {
    NROM(NROM),
    MMC1(MMC1),
    UxROM(UxROM),
    CNROM(CNROM),
    GxROM(GxROM),
}

impl MapperOperations for Mapper {

    fn read_prg(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        match cart.mapper {
            Mapper::NROM(_) => NROM::read_prg(cart, addr),
            Mapper::MMC1(_) => MMC1::read_prg(cart, addr),
            Mapper::UxROM(_) => UxROM::read_prg(cart, addr),
            Mapper::CNROM(_) => CNROM::read_prg(cart, addr),
            Mapper::GxROM(_) => GxROM::read_prg(cart, addr),
        }
    }

    fn write_prg(cart: &mut Cartridge, addr: u16, data: u8) -> Result<(), ()>{
        match cart.mapper {
            Mapper::NROM(_) => NROM::write_prg(cart, addr, data),
            Mapper::MMC1(_) => MMC1::write_prg(cart, addr, data),
            Mapper::UxROM(_) => UxROM::write_prg(cart, addr, data),
            Mapper::CNROM(_) => CNROM::write_prg(cart, addr, data),
            Mapper::GxROM(_) => GxROM::write_prg(cart, addr, data),
        }
    }

    fn read_chr(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        match cart.mapper {
            Mapper::NROM(_) => NROM::read_chr(cart, addr),
            Mapper::MMC1(_) => MMC1::read_chr(cart, addr),
            Mapper::UxROM(_) => UxROM::read_chr(cart, addr),
            Mapper::CNROM(_) => CNROM::read_chr(cart, addr),
            Mapper::GxROM(_) => GxROM::read_chr(cart, addr),
        }
    }

    fn write_chr(cart: &mut Cartridge, addr: u16, data :u8) -> Result<(), ()> {
        match cart.mapper {
            Mapper::NROM(_) => NROM::write_chr(cart, addr, data),
            Mapper::MMC1(_) => MMC1::write_chr(cart, addr, data),
            Mapper::UxROM(_) => UxROM::write_chr(cart, addr, data),
            Mapper::CNROM(_) => CNROM::write_chr(cart, addr, data),
            Mapper::GxROM(_) => GxROM::write_chr(cart, addr, data),
        }
    }

}