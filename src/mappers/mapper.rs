use crate::cartridge::Cartridge;

pub struct Mapper {
    pub read_prg: fn(cart: &Cartridge, addr: u16) -> u16,
    pub write_prg: fn(cart: &Cartridge, addr: u16) -> u16,
    pub read_chr: fn(cart: &Cartridge, addr: u16) -> u16,
    pub write_chr: fn(cart: &Cartridge, addr: u16) -> u16,
}