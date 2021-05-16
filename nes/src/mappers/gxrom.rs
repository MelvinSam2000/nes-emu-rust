use crate::mappers::mapper::Mapper;
use crate::cartridge::Cartridge;

pub const GxROM: Mapper = Mapper {
    read_prg,
    write_prg,
    read_chr,
    write_chr
};


fn read_prg(cart: &mut Cartridge, addr: u16) -> u8 {
    if addr >= 0x8000 {
        let mapped_addr = (cart.gxrom_banksel.0 as u16)*0x8000 + (addr & 0x7fff);
        return cart.prgmem[mapped_addr as usize];
    }
    return 0;
}

fn write_prg(cart: &mut Cartridge, addr: u16, data: u8) {
    match addr {
        0x8000..=0xffff => {
            cart.gxrom_banksel = (data & 0x03, (data & 0x30) >> 4);
        },
        _ => {}
    };
}

fn read_chr(cart: &mut Cartridge, addr: u16) -> u8 {
    if addr < 0x2000 {
        let mapped_addr = (cart.gxrom_banksel.1 as u16)*0x2000 + addr;
        return cart.chrmem[mapped_addr as usize];
    }
    return 0;
}

fn write_chr(cart: &mut Cartridge, addr: u16, data: u8) {
    panic!("Invalid write");
}

