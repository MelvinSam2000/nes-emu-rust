extern crate rand;

use crate::nes::Nes;
use crate::busppu;
use crate::ppu;
use crate::ppu::regcontrol::RegControl;
use crate::ppu::regmask::RegMask;
use crate::ppu::regstatus::RegStatus;
use crate::ppu::regaddrdata::RegAddrData;
use crate::ppu::regloopy::RegLoopy;
use crate::events::drawevent::DrawEvent;
use crate::cpu::cpu;

pub struct Ppu {
    pub oam: [u8; 256],
    // screen scanning
    pub scan_line: u16,
    pub scan_cycle: u16,
    // ppu registers for cpu communication
    pub reg_control: RegControl,
    pub reg_mask: RegMask,
    pub reg_status: RegStatus,
    pub reg_addr_data: RegAddrData,
    pub vram_addr: RegLoopy,
    pub tram_addr: RegLoopy,
}

impl Ppu {

    pub fn new() -> Self {
        return Self {
            oam: [0; 256],

            scan_line: 0,
            scan_cycle: 0,

            reg_control: RegControl::new(),
            reg_mask: RegMask::new(),
            reg_status: RegStatus::new(),
            reg_addr_data: RegAddrData::new(),
            vram_addr: RegLoopy::new(),
            tram_addr: RegLoopy::new(),
        };
    }
}

pub static PALETTE_TO_RGB: [(u8,u8,u8); 64] = [
   (0x80, 0x80, 0x80), (0x00, 0x3D, 0xA6), (0x00, 0x12, 0xB0), (0x44, 0x00, 0x96), (0xA1, 0x00, 0x5E),
   (0xC7, 0x00, 0x28), (0xBA, 0x06, 0x00), (0x8C, 0x17, 0x00), (0x5C, 0x2F, 0x00), (0x10, 0x45, 0x00),
   (0x05, 0x4A, 0x00), (0x00, 0x47, 0x2E), (0x00, 0x41, 0x66), (0x00, 0x00, 0x00), (0x05, 0x05, 0x05),
   (0x05, 0x05, 0x05), (0xC7, 0xC7, 0xC7), (0x00, 0x77, 0xFF), (0x21, 0x55, 0xFF), (0x82, 0x37, 0xFA),
   (0xEB, 0x2F, 0xB5), (0xFF, 0x29, 0x50), (0xFF, 0x22, 0x00), (0xD6, 0x32, 0x00), (0xC4, 0x62, 0x00),
   (0x35, 0x80, 0x00), (0x05, 0x8F, 0x00), (0x00, 0x8A, 0x55), (0x00, 0x99, 0xCC), (0x21, 0x21, 0x21),
   (0x09, 0x09, 0x09), (0x09, 0x09, 0x09), (0xFF, 0xFF, 0xFF), (0x0F, 0xD7, 0xFF), (0x69, 0xA2, 0xFF),
   (0xD4, 0x80, 0xFF), (0xFF, 0x45, 0xF3), (0xFF, 0x61, 0x8B), (0xFF, 0x88, 0x33), (0xFF, 0x9C, 0x12),
   (0xFA, 0xBC, 0x20), (0x9F, 0xE3, 0x0E), (0x2B, 0xF0, 0x35), (0x0C, 0xF0, 0xA4), (0x05, 0xFB, 0xFF),
   (0x5E, 0x5E, 0x5E), (0x0D, 0x0D, 0x0D), (0x0D, 0x0D, 0x0D), (0xFF, 0xFF, 0xFF), (0xA6, 0xFC, 0xFF),
   (0xB3, 0xEC, 0xFF), (0xDA, 0xAB, 0xEB), (0xFF, 0xA8, 0xF9), (0xFF, 0xAB, 0xB3), (0xFF, 0xD2, 0xB0),
   (0xFF, 0xEF, 0xA6), (0xFF, 0xF7, 0x9C), (0xD7, 0xE8, 0x95), (0xA6, 0xED, 0xAF), (0xA2, 0xF2, 0xDA),
   (0x99, 0xFF, 0xFC), (0xDD, 0xDD, 0xDD), (0x11, 0x11, 0x11), (0x11, 0x11, 0x11)
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

    nes.ppu.scan_cycle += 1;
    
    if nes.ppu.scan_cycle == 341 {
        nes.ppu.scan_cycle = 0;
        nes.ppu.scan_line += 1;
    }

    if nes.ppu.scan_line == 262 {
        nes.ppu.scan_line = 0;
        nes.ppu.reg_status.set_vblank(false);
    }

    if nes.ppu.scan_line == 241 {
        if nes.ppu.reg_control.is_nmi_enabled() {
            nes.ppu.reg_status.set_vblank(true);
            cpu::nmi(nes);
        }
    }



    if nes.ppu.scan_line < 240 && nes.ppu.scan_cycle < 256 {
        /*
        let (i, j) = (nes.ppu.scan_line as u8, nes.ppu.scan_cycle as u8);
        nes.submit_draw_event(DrawEvent {
            position: (i, j),
            rgb: if rand::random() { (255, 255, 255) } else { (0, 0, 0) },
        });
        */
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
            // remove this later
            nes.ppu.reg_status.set_vblank(true);

            let data = nes.ppu.reg_status.reg;
            nes.ppu.reg_status.set_vblank(false);
            nes.ppu.reg_addr_data.set_latch(false);
            return data;
        },
        OAMADDR => {

        },
        OAMDATA => {

        },
        PPUDATA => {
            return ppu::regaddrdata::read_data(nes);
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

        },
        OAMADDR => {

        },
        OAMDATA => {

        },
        PPUADDR => {
            return ppu::regaddrdata::write_addr(nes, data);
        },
        PPUDATA => {

        }
        _ => {
            return;
        }
    }

    
}

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
                    let rgb = match pixel {
                        0 => (0, 0, 0),
                        1 => (255, 0, 0),
                        2 => (0, 255, 0),
                        3 => (0, 0, 255),
                        _ => (255, 0, 255)
                    };
                    nes.submit_draw_event(DrawEvent { position: (
                        
                        
                        (tile_y * 8 + (7 - col)) as u8, 
                        (tile_x * 8 + row) as u8,
                    ), rgb})
                }
            }
        }
    }
}

pub fn show_tile(nes: &mut Nes, bank: u16, tile_n: u16) {
    assert!(bank <= 1);

    let bank = bank * 0x1000;
 
    for y in 0..8 {
        let offset: u16 = bank + tile_n * 16;
        read(nes, offset);
        let mut upper = read(nes, offset);
        let mut lower = read(nes, offset + 8);
 
        for x in (0..=7).rev() {
            let value = (1 & upper) << 1 | (1 & lower);
            upper = upper >> 1;
            lower = lower >> 1;
            let rgb = match value {
                0 => PALETTE_TO_RGB[0x01],
                1 => PALETTE_TO_RGB[0x23],
                2 => PALETTE_TO_RGB[0x27],
                3 => PALETTE_TO_RGB[0x30],
                _ => panic!("can't be"),
            };
            nes.submit_draw_event(DrawEvent {position: (x + 16*tile_n as u8 % 8, y), rgb});
        }
    }
 }