use crate::nes::Nes;
use crate::cartridge::Mirroring;

pub struct BusPpu {
    pub vram: [u8; 0x1000],
    pub palette: [u8; 0x20],
}

impl BusPpu {
    
    pub fn new() -> Self {
        return Self {
            vram: [0; 0x1000],
            palette: [0; 0x20]
        };
    }
}

pub fn read(nes: &mut Nes, addr: u16) -> u8 {

    match addr {
        0x0000 ..= 0x1fff => {
            return nes.cartridge.chr_read(addr);
        },
        0x2000 ..= 0x3eff => {
            let mut mapped_addr = addr & 0x0fff;
            match nes.cartridge.mirroring {
                Mirroring::HORIZONTAL => {
                    if mapped_addr >= 0x0400 && mapped_addr < 0x0800 {
                        mapped_addr -= 0x0400;
                    } else if mapped_addr >= 0x0c00 {
                        mapped_addr -= 0x0400;
                    }
                },
                Mirroring::VERTICAL => {
                    if mapped_addr >= 0x0800 && mapped_addr < 0x0c00 {
                        mapped_addr -= 0x0800;
                    } else if mapped_addr >= 0x0c00 {
                        mapped_addr -= 0x0800;
                    }
                }
            }
            return nes.busppu.vram[mapped_addr as usize];
        },
        0x3f00 ..= 0x3f1f => {
            return nes.busppu.palette[(addr & !0x3f00) as usize];
        },
        0x3f20 ..= 0x3fff => {
            
            let mut mapped = addr & 0x001f;
            if mapped == 0x0010 {
                mapped = 0x0000;
            }
            if mapped == 0x0014 {
                mapped = 0x0004;
            }
            if mapped == 0x0018 {
                mapped = 0x0008;
            }
            if mapped == 0x001C {
                mapped = 0x000C;
            }
            
            //let mapped = (addr - 0x3f20) & 0x001f;
            return nes.busppu.palette[mapped as usize];
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
        0x2000 ..= 0x3eff => {
            let mut mapped_addr = addr & 0x0fff;
            match nes.cartridge.mirroring {
                Mirroring::HORIZONTAL => {
                    if mapped_addr >= 0x0400 && mapped_addr < 0x0800 {
                        mapped_addr -= 0x0400;
                    } else if mapped_addr >= 0x0c00 {
                        mapped_addr -= 0x0400;
                    }
                },
                Mirroring::VERTICAL => {
                    if mapped_addr >= 0x0800 && mapped_addr < 0x0c00 {
                        mapped_addr -= 0x0800;
                    } else if mapped_addr >= 0x0c00 {
                        mapped_addr -= 0x0800;
                    }
                }
            }
            nes.busppu.vram[mapped_addr as usize] = data;
        },
        0x3f00 ..= 0x3f1f => {
            nes.busppu.palette[(addr & !0x3f00) as usize] = data;
        },
        0x3f20 ..= 0x3fff => {
            
            let mut mapped = addr & 0x001f;
            if mapped == 0x0010 {
                mapped = 0x0000;
            }
            if mapped == 0x0014 {
                mapped = 0x0004;
            }
            if mapped == 0x0018 {
                mapped = 0x0008;
            }
            if mapped == 0x001C {
                mapped = 0x000C;
            }

		    nes.busppu.palette[mapped as usize] = data;
            
            //let mapped = (addr - 0x3f20) & 0x001f;
            //nes.busppu.palette[mapped as usize] = data;
        },
        _ => {
            return;
        }
    }
}
