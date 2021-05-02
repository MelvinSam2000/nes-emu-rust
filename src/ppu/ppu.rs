extern crate rand;

use crate::nes::Nes;
use crate::busppu;
use crate::ppu::regcontrol::RegControl;
use crate::ppu::regmask::RegMask;
use crate::ppu::regstatus::RegStatus;
use crate::events::drawevent::DrawEvent;
use crate::cpu::cpu;

pub struct Ppu {
    pub oam: [u8; 256],
    // screen scanning
    pub scan_line: i16,
    pub scan_cycle: u16,
    // ppu registers for cpu communication
    pub reg_control: RegControl,
    pub reg_mask: RegMask,
    pub reg_status: RegStatus,
    pub reg_addr: u16,
    pub reg_data: u8,
    pub addr_latch: bool,
}

impl Ppu {

    pub fn new() -> Self {
        return Self {
            oam: [0; 256],

            scan_line: -1,
            scan_cycle: 0,

            reg_control: RegControl::new(),
            reg_mask: RegMask::new(),
            reg_status: RegStatus::new(),

            reg_addr: 0x0000,
            reg_data: 0x00,
            addr_latch: true,
        };
    }
}

pub static PALETTE_TO_RGB: [(u8, u8, u8); 64] = [
   (0x80, 0x80, 0x80), (0x00, 0x3d, 0xa6), (0x00, 0x12, 0xb0), (0x44, 0x00, 0x96), (0xa1, 0x00, 0x5e),
   (0xc7, 0x00, 0x28), (0xba, 0x06, 0x00), (0x8c, 0x17, 0x00), (0x5c, 0x2f, 0x00), (0x10, 0x45, 0x00),
   (0x05, 0x4a, 0x00), (0x00, 0x47, 0x2e), (0x00, 0x41, 0x66), (0x00, 0x00, 0x00), (0x05, 0x05, 0x05),
   (0x05, 0x05, 0x05), (0xc7, 0xc7, 0xc7), (0x00, 0x77, 0xff), (0x21, 0x55, 0xff), (0x82, 0x37, 0xfa),
   (0xeb, 0x2f, 0xb5), (0xff, 0x29, 0x50), (0xff, 0x22, 0x00), (0xd6, 0x32, 0x00), (0xc4, 0x62, 0x00),
   (0x35, 0x80, 0x00), (0x05, 0x8f, 0x00), (0x00, 0x8a, 0x55), (0x00, 0x99, 0xcc), (0x21, 0x21, 0x21),
   (0x09, 0x09, 0x09), (0x09, 0x09, 0x09), (0xff, 0xff, 0xff), (0x0f, 0xd7, 0xff), (0x69, 0xa2, 0xff),
   (0xd4, 0x80, 0xff), (0xff, 0x45, 0xf3), (0xff, 0x61, 0x8b), (0xff, 0x88, 0x33), (0xff, 0x9c, 0x12),
   (0xfa, 0xbc, 0x20), (0x9f, 0xe3, 0x0e), (0x2b, 0xf0, 0x35), (0x0c, 0xf0, 0xa4), (0x05, 0xfb, 0xff),
   (0x5e, 0x5e, 0x5e), (0x0d, 0x0d, 0x0d), (0x0d, 0x0d, 0x0d), (0xff, 0xff, 0xff), (0xa6, 0xfc, 0xff),
   (0xb3, 0xec, 0xff), (0xda, 0xab, 0xeb), (0xff, 0xa8, 0xf9), (0xff, 0xab, 0xb3), (0xff, 0xd2, 0xb0),
   (0xff, 0xef, 0xa6), (0xff, 0xf7, 0x9c), (0xd7, 0xe8, 0x95), (0xa6, 0xed, 0xaf), (0xa2, 0xf2, 0xda),
   (0x99, 0xff, 0xfc), (0xdd, 0xdd, 0xdd), (0x11, 0x11, 0x11), (0x11, 0x11, 0x11)
];


const PPUCTRL: u16 = 0x2000;
const PPUMASK: u16 = 0x2001;
const PPUSTATUS: u16 = 0x2002;
const OAMADDR: u16 = 0x2003;
const OAMDATA: u16 = 0x2004;
const PPUSCROLL: u16 = 0x2005;
const PPUADDR: u16 = 0x2006;
const PPUDATA: u16 = 0x2007;


pub fn clock(nes: &mut Nes) {

    // Enter VBLANK
    if nes.ppu.scan_line == 241 && nes.ppu.scan_cycle == 1 {
        nes.ppu.reg_status.set_vblank(true);
        if nes.ppu.reg_control.is_nmi_enabled() {
            render(nes);
            cpu::nmi(nes);
        }
    }

    nes.ppu.scan_cycle += 1;
	if nes.ppu.scan_cycle >= 341 {
		nes.ppu.scan_cycle = 0;
		nes.ppu.scan_line += 1;
		if nes.ppu.scan_line >= 261 {
			nes.ppu.scan_line = -1;
		}
	}
    
}

pub fn read(nes: &mut Nes, addr: u16) -> u8 {
    return busppu::read(nes, addr);
}

pub fn write(nes: &mut Nes, addr: u16, data: u8) {
    busppu::write(nes, addr, data);
}

pub fn read_ppu_reg(nes: &mut Nes, addr: u16) -> u8 {
    match addr {
        PPUCTRL | PPUMASK | PPUSCROLL | PPUADDR => {
            // these registers are write only
            return 0x00;
        },
        PPUSTATUS => {
            let data = nes.ppu.reg_status.reg;
            nes.ppu.reg_status.set_vblank(false);
            nes.ppu.addr_latch = true;
            return data;
        },
        OAMADDR => {

        },
        OAMDATA => {

        },
        PPUDATA => {
            
            let maddr = nes.ppu.reg_addr;
            nes.ppu.reg_addr = nes.ppu.reg_addr.wrapping_add(
                if nes.ppu.reg_control.is_inc_mode() {32} else {1});

            match maddr {
                0x0000..=0x2fff => {
                    let output = nes.ppu.reg_data;
                    nes.ppu.reg_data = read(nes, maddr);
                    return output;
                },
                0x3f00..=0x3fff => {
                    return read(nes, maddr);
                },
                _ => {
                    return 0x00;
                }
            }
        }
        _ => {
            return 0x00;
        }
    }
    return 0x00;
}

pub fn write_ppu_reg(nes: &mut Nes, addr: u16, data: u8) {
    match addr {
        PPUSTATUS => {
            // This register is read only
            return;
        },
        PPUCTRL => {
            nes.ppu.reg_control.reg = data;
        },
        PPUMASK => {
            nes.ppu.reg_mask.reg = data;
        },
        PPUSCROLL => {
            /*
            if nes.ppu.addr_latch {
                nes.ppu.fine_x = data & 0x07;
                nes.ppu.loopy_t.set_coarse_x(data >> 3);
            } else {
                nes.ppu.loopy_t.set_fine_y(data & 0x07);
                nes.ppu.loopy_t.set_coarse_x(data >> 3);
            }
            nes.ppu.reg_addr_data.flip_latch();
            */
        },
        OAMADDR => {

        },
        OAMDATA => {

        },
        PPUADDR => {
            if nes.ppu.addr_latch {
                // set high byte
                nes.ppu.reg_addr = (nes.ppu.reg_addr & 0x00ff) | (data as u16) << 8;
            } else {
                // set low byte
                nes.ppu.reg_addr = (nes.ppu.reg_addr & 0xff00) | data as u16;
            }

            nes.ppu.reg_addr &= 0x3fff;

            nes.ppu.addr_latch = !nes.ppu.addr_latch;
        },
        PPUDATA => {
            write(nes, nes.ppu.reg_addr, data);
            // TODO
            nes.ppu.reg_addr = nes.ppu.reg_addr.wrapping_add(
                if nes.ppu.reg_control.is_inc_mode() {32} else {1});
        }
        _ => {
            return;
        }
    }

    
}

pub fn render(nes: &mut Nes) {

    let palette_bank = nes.ppu.reg_control.get_bg() as u16;

    // First nametable
    for i in 0x2000..=0x23bf {
        // get tile ID from vram
        let tile = read(nes, i);
        let tile_col = i % 32;
        let tile_row = i / 32;
        
        // Draw tile
        for row in 0..8 {
            let mut tile_lsb = read(nes, palette_bank*0x1000 + (tile as u16)*16 + row);
            let mut tile_msb = read(nes, palette_bank*0x1000 + (tile as u16)*16 + row + 8);
            for col in 0..8 {
                let pixel = (tile_msb & 0x01) + (tile_lsb & 0x01);
                tile_lsb >>= 1;
                tile_msb >>= 1;

                //let rgb = if pixel == 0 { (0, 0, 0) } else { (255, 255, 255) };
                let rgb = PALETTE_TO_RGB[(read(nes, 0x3f00 + pixel as u16) % 64) as usize];
                //let rgb = (0, 0, (((tile as u16)*101)  % 255) as u8);

                nes.submit_draw_event(DrawEvent { position: (
                    (tile_col * 8 + (7 - col)) as u8, 
                    (tile_row * 8 + row) as u8,
                ), rgb})
            }
        }

    }
}

// TESTING FUNCTIONS

pub fn draw_chr(nes: &mut Nes, bank: u16) {
    
    for tile_x in 0..16 {
        for tile_y in 0..16 {
            let offset = tile_x*256 + tile_y*16;
            for row in 0..8 {
                let mut tile_lsb = read(nes, bank*0x1000 + offset + row);
                let mut tile_msb = read(nes, bank*0x1000 + offset + row + 8);
                for col in 0..8 {
                    let pixel = (tile_msb & 0x01) + (tile_lsb & 0x01);
                    tile_lsb >>= 1;
                    tile_msb >>= 1;

                    //let rgb = if pixel == 0 { (0, 0, 0) } else { (255, 255, 255) };
                    let rgb = PALETTE_TO_RGB[(read(nes, 0x3f00 + pixel as u16) % 64) as usize];
                    
                    nes.submit_draw_event(DrawEvent { position: (
                        (tile_y * 8 + (7 - col)) as u8, 
                        (tile_x * 8 + row) as u8,
                    ), rgb})
                }
            }
        }
    }
}

pub fn get_palette_tbl(nes: &mut Nes) {
    println!("PALETTE... ");
    for addr in 0x3f00..=0x3f1f {
        let data = read(nes, addr);
        let (r, g, b) = PALETTE_TO_RGB[data as usize];
        println!("{:#04X}: {:#04X}; RGB: 0x{:02X}{:02X}{:02X}", 
            addr, data, r, g, b);
    }
}