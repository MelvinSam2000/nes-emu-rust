use crate::cartridge::Cartridge;

pub trait Mapper {
    fn read_prg(&self, cart: &Cartridge, addr: u16) -> u16;
    fn write_prg(&self, cart: &Cartridge, addr: u16) -> u16;
    fn read_chr(&self, cart: &Cartridge, addr: u16) -> u16;
    fn write_chr(&self, cart: &Cartridge, addr: u16) -> u16;
}