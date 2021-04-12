use crate::nes::Nes;

pub struct BusPpu {
    pub nametbl: [u8; 0x1000],
    pub palette: [u8; 0x20],
}

impl BusPpu {
    
    pub fn new() -> Self {
        return Self {
            nametbl: [0; 0x1000],
            palette: [0; 0x20]
        };
    }
}

pub fn read(nes: &mut Nes, addr: u16) -> u8 {

    match addr {
        0x0000 ..= 0x1fff => {
            return nes.cartridge.chr_read(addr);
        },
        0x2000 ..= 0x2fff => {
            return nes.busppu.nametbl[(addr & 0x0fff) as usize];
        },
        0x3000 ..= 0x3eff => {
            return nes.busppu.nametbl[(addr & 0x0fff) as usize];
        },
        0x3f00 ..= 0x3f1f => {
            return nes.busppu.palette[(addr & !0x3f00) as usize];
        },
        0x3f20 ..= 0x3fff => {
            return nes.busppu.palette[(addr & !0x3f00) as usize];
        },
        _ => {
            return 0x00;
        }
    }
}

pub fn write(nes: &mut Nes, addr: u16, data: u8) {

    match addr {
        0x0000 ..= 0x1fff => {
            nes.cartridge.chr_write(addr, data);
        },
        0x2000 ..= 0x2fff => {
            nes.busppu.nametbl[(addr & 0x0fff) as usize] = data;
        },
        0x3000 ..= 0x3eff => {
            nes.busppu.nametbl[(addr & 0x0fff) as usize] = data;
        },
        0x3f00 ..= 0x3f1f => {
            nes.busppu.palette[(addr & !0x3f00) as usize] = data;
        },
        0x3f20 ..= 0x3fff => {
            nes.busppu.palette[(addr & !0x3f00) as usize] = data;
        },
        _ => {
            return;
        }
    }
}
