use crate::mappers::mapper::Mapper;
use crate::cartridge::Cartridge;

pub const CNROM: Mapper = Mapper {
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
    match addr {
        0x8000..=0xffff => {
            cart.cnrom_banksel = (data & 0x03) as u8;
        },
        _ => {}
    };
}

fn read_chr(cart: &mut Cartridge, addr: u16) -> u8 {
    if addr < 0x2000 {
        let mapped_addr = (cart.cnrom_banksel as u16)*0x2000 + addr;
        return cart.chrmem[mapped_addr as usize];
    }
    return 0;
}

fn write_chr(cart: &mut Cartridge, addr: u16, data: u8) {
    //cart.chrmem[addr as usize] = data;
    panic!("Invalid write...");
}

