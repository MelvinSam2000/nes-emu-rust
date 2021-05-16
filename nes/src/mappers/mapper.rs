use crate::cartridge::Cartridge;

pub struct Mapper {
    pub read_prg: fn(cart: &mut Cartridge, addr: u16) -> u8,
    pub write_prg: fn(cart: &mut Cartridge, addr: u16, data: u8),
    pub read_chr: fn(cart: &mut Cartridge, addr: u16) -> u8,
    pub write_chr: fn(cart: &mut Cartridge, addr: u16, data :u8),
}