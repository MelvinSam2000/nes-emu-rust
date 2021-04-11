use crate::nes::Nes;

pub struct BusCpu {
    pub ram: [u8; 0x2000]
}

impl BusCpu {
    
    pub fn new() -> Self {
        return Self {
            ram: [0; 0x2000]
        };
    }
}

pub fn read(nes: &mut Nes, addr: u16) -> u8 {

    match addr {
        0x0000 ..= 0x1fff => {
            return nes.buscpu.ram[addr as usize];
        },
        0x2000 ..= 0x3fff => {
            println!("READING FROM PPU UNSUPPORTED");
            return 0;
        },
        0x4000 ..= 0x401f => {
            println!("READING FROM APU and IO UNSUPPORTED");
            return 0;
        },
        0x4020 ..= 0xffff => {
            return nes.cartridge.prg_read(addr);
        }
    }
}

pub fn write(nes: &mut Nes, addr: u16, data: u8) {

    match addr {
        0x0000 ..= 0x1fff => {
            nes.buscpu.ram[addr as usize] = data;
        },
        0x2000 ..= 0x3fff => {
            println!("WRITING TO PPU UNSUPPORTED");
        },
        0x4000 ..= 0x401f => {
            println!("WRITING TO APU and IO UNSUPPORTED");
        },
        0x4020 ..= 0xffff => {
            nes.cartridge.prg_write(addr, data);
        }
    }
}
