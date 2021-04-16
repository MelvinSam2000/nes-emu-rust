use crate::mappers::mapper::Mapper;
use crate::cartridge::Cartridge;

pub const NROM: Mapper = Mapper {
    read_prg,
    write_prg,
    read_chr,
    write_chr
};


fn read_prg(cart: &Cartridge, addr: u16) -> u16 {
    if 0x8000 <= addr {
        if cart.prg_banks == 2 {
            return addr & 0x7fff;
        }
        if cart.prg_banks == 1 {
            return addr & 0x3fff;
        }
    }
    return addr;
}

fn write_prg(cart: &Cartridge, addr: u16) -> u16 {
    if 0x8000 <= addr {
        if cart.prg_banks == 2 {
            return addr & 0x7fff;
        }
        if cart.prg_banks == 1 {
            return addr & 0x3fff;
        }
    }
    return addr;
}

fn read_chr(cart: &Cartridge, addr: u16) -> u16 {
    return addr;
}

fn write_chr(cart: &Cartridge, addr: u16) -> u16 {
    return addr;
}
