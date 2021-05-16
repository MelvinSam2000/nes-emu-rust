use crate::mappers::mapper::Mapper;
use crate::cartridge::Cartridge;

pub const NROM: Mapper = Mapper {
    read_prg,
    write_prg,
    read_chr,
    write_chr
};


fn read_prg(cart: &mut Cartridge, addr: u16) -> u8 {
    let mut mapped_addr = 0;
    if 0x8000 <= addr {
        if cart.prg_banks == 2 {
            mapped_addr = addr & 0x7fff;
        }
        if cart.prg_banks == 1 {
            mapped_addr = addr & 0x3fff;
        }
    }
    return cart.prgmem[mapped_addr as usize];
}

fn write_prg(cart: &mut Cartridge, addr: u16, data: u8) {
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
}

fn read_chr(cart: &mut Cartridge, addr: u16) -> u8 {
    return cart.chrmem[addr as usize];
}

fn write_chr(cart: &mut Cartridge, addr: u16, data: u8) {
    cart.chrmem[addr as usize] = data;
}
