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
    pub scan_line: i16,
    pub scan_cycle: u16,
    // ppu registers for cpu communication
    pub reg_control: RegControl,
    pub reg_mask: RegMask,
    pub reg_status: RegStatus,
    pub reg_addr_data: RegAddrData,
    // helper variables
    pub loopy_v: RegLoopy,
    pub loopy_t: RegLoopy,
    pub fine_x: u8,
    pub bg_next_tile_id: u8,
    pub bg_next_tile_attr: u8,
    pub bg_next_tile_lsb: u8,
    pub bg_next_tile_msb: u8,
    pub bg_shift_pat_lo: u16,
    pub bg_shift_pat_hi: u16,
    pub bg_shift_att_lo: u16,
    pub bg_shift_att_hi: u16,
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
            reg_addr_data: RegAddrData::new(),
            
            loopy_v: RegLoopy::new(),
            loopy_t: RegLoopy::new(),
            fine_x: 0x00,

            bg_next_tile_id: 0x00,
            bg_next_tile_attr: 0x00,
            bg_next_tile_lsb: 0x00,
            bg_next_tile_msb: 0x00,

            bg_shift_pat_lo: 0x0000,
            bg_shift_pat_hi: 0x0000,
            bg_shift_att_lo: 0x0000,
            bg_shift_att_hi: 0x0000,
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

    if nes.ppu.scan_line >= -1 && nes.ppu.scan_line < 240 {

        if nes.ppu.scan_line == 0 && nes.ppu.scan_cycle == 0 {
            nes.ppu.scan_cycle = 1;
        }

        if nes.ppu.scan_line == -1 && nes.ppu.scan_cycle == 1 {
            nes.ppu.reg_status.set_vblank(false);
        }

        if (nes.ppu.scan_cycle >= 2 && nes.ppu.scan_cycle < 258) || (nes.ppu.scan_cycle >= 321 && nes.ppu.scan_cycle < 338) {
            
            update_shifters(nes);

            match (nes.ppu.scan_cycle - 1) % 8 {
                0 => {
                    load_bg_shifters(nes);
                    nes.ppu.bg_next_tile_id = read(nes, 0x2000 | (nes.ppu.loopy_v.reg & 0x0fff));
                },
                2 => {
                    nes.ppu.bg_next_tile_attr = read(nes, 
                        0x23c0 |
                        ((nes.ppu.loopy_v.get_nametable_y() as u16) << 11) |
                        ((nes.ppu.loopy_v.get_nametable_x() as u16) << 10) |
                        ((nes.ppu.loopy_v.get_coarse_y() >> 2) << 3) |
                        ((nes.ppu.loopy_v.get_coarse_x() >> 2))
                    );
                    if nes.ppu.loopy_v.get_coarse_y() & 0x02 != 0 {
                        nes.ppu.bg_next_tile_attr >>= 4;
                    }
                    if nes.ppu.loopy_v.get_coarse_x() & 0x02 != 0 {
                        nes.ppu.bg_next_tile_attr >>= 2;
                    }
                    nes.ppu.bg_next_tile_attr &= 0x0003;
                },
                4 => {
                    nes.ppu.bg_next_tile_lsb = read(nes, 
                        ((nes.ppu.reg_control.get_bg() as u16) << 12)
                        .wrapping_add((nes.ppu.bg_next_tile_id as u16) << 4)
                        .wrapping_add(nes.ppu.loopy_v.get_fine_y()));
                },
                6 => {
                    nes.ppu.bg_next_tile_msb = read(nes, 
                        ((nes.ppu.reg_control.get_bg() as u16) << 12)
                        .wrapping_add((nes.ppu.bg_next_tile_id as u16) << 4)
                        .wrapping_add(nes.ppu.loopy_v.get_fine_y())
                        .wrapping_add(8));
                },
                7 => {
                    incr_scroll_x(nes);
                },
                _ => {}
            }

            if nes.ppu.scan_cycle == 256 {
                incr_scroll_y(nes);
            }

            if nes.ppu.scan_cycle == 257 {
                load_bg_shifters(nes);
                transfer_addr_x(nes);
            }

            if nes.ppu.scan_cycle == 338 || nes.ppu.scan_cycle == 340 {
                nes.ppu.bg_next_tile_id = read(nes, 0x2000 | (nes.ppu.loopy_v.reg & 0x0fff));
            }

            if nes.ppu.scan_line == -1 && nes.ppu.scan_cycle >= 280 && nes.ppu.scan_cycle < 305 {
                transfer_addr_y(nes);
            }   
        }
    }

    // Enter VBLANK
    if nes.ppu.scan_line >= 241 && nes.ppu.scan_line < 261 {
        if nes.ppu.scan_line == 241 && nes.ppu.scan_cycle == 1 {
            nes.ppu.reg_status.set_vblank(true);
            if nes.ppu.reg_control.is_nmi_enabled() {
                cpu::nmi(nes);
            }
        }
    }

    
    if nes.ppu.reg_mask.render_bg_enabled() 
        && nes.ppu.scan_cycle > 0 && nes.ppu.scan_cycle <= 256
        && nes.ppu.scan_line >= 0 && nes.ppu.scan_line < 240  {
        let bit_mux = 0x8000 >> nes.ppu.fine_x;
        let mut pixel = (nes.ppu.bg_shift_pat_lo & bit_mux != 0) as u8;
        pixel |= ((nes.ppu.bg_shift_pat_hi & bit_mux != 0) as u8) << 1;
        let mut palette = (nes.ppu.bg_shift_att_lo & bit_mux != 0) as u8;
        palette |= ((nes.ppu.bg_shift_att_hi & bit_mux != 0) as u8) << 1;

        let rgb = PALETTE_TO_RGB[(read(nes, 0x3F00 + ((palette << 2) + pixel) as u16) & 0x3F) as usize];

        nes.submit_draw_event(DrawEvent {
            position: ((nes.ppu.scan_cycle as u8).wrapping_sub(1), nes.ppu.scan_line as u8), rgb
        });
    }

    nes.ppu.scan_cycle += 1;
	if nes.ppu.scan_cycle >= 341 {
		nes.ppu.scan_cycle = 0;
		nes.ppu.scan_line += 1;
		if nes.ppu.scan_line >= 261 {
			nes.ppu.scan_line = -1;
			//frame_complete = true;
		}
	}



    // OLD
    /*
    nes.ppu.scan_cycle += 1;
    
    // Reset scan cycle counter
    if nes.ppu.scan_cycle == 341 {
        nes.ppu.scan_cycle = 0;
        nes.ppu.scan_line += 1;
    }

    // Reset scan line counter
    if nes.ppu.scan_line == 262 {
        nes.ppu.scan_line = 0;
        nes.ppu.reg_status.set_vblank(false);
    }

    // Enter VBLANK
    if nes.ppu.scan_line == 242 && nes.ppu.scan_cycle == 1 {
        nes.ppu.reg_status.set_vblank(true);
        if nes.ppu.reg_control.is_nmi_enabled() {
            cpu::nmi(nes);
        }
    }

    // Advance and execute scan line operations
    if nes.ppu.scan_line <= 241 {
        let cycle = nes.ppu.scan_cycle;
        update_shifters(nes);
        if (cycle >= 2 && cycle < 258) || (cycle >= 321 && cycle < 338) {
            match (cycle - 1) % 8 {
                0 => {
                    load_bg_shifters(nes);
                    nes.ppu.bg_next_tile_id = read(nes, 0x2000 | (nes.ppu.loopy_v.reg & 0x0fff));
                },
                2 => {
                    nes.ppu.bg_next_tile_attr = read(nes, 
                        0x23c0 |
                        ((nes.ppu.loopy_v.get_nametable_y() as u16) << 11) |
                        ((nes.ppu.loopy_v.get_nametable_x() as u16) << 10) |
                        ((nes.ppu.loopy_v.get_coarse_y() >> 2) << 3) |
                        ((nes.ppu.loopy_v.get_coarse_x() >> 2))
                    );
                    if nes.ppu.loopy_v.get_coarse_y() & 0x02 != 0 {
                        nes.ppu.bg_next_tile_attr >>= 4;
                    }
                    if nes.ppu.loopy_v.get_coarse_x() & 0x02 != 0 {
                        nes.ppu.bg_next_tile_attr >>= 2;
                    }
                    nes.ppu.bg_next_tile_attr &= 0x0003;
                },
                4 => {
                    nes.ppu.bg_next_tile_lsb = read(nes, 
                        ((nes.ppu.reg_control.get_bg() as u16) << 12)
                        .wrapping_add((nes.ppu.bg_next_tile_id as u16) << 4)
                        .wrapping_add(nes.ppu.loopy_v.get_fine_y()));
                },
                6 => {
                    nes.ppu.bg_next_tile_msb = read(nes, 
                        ((nes.ppu.reg_control.get_bg() as u16) << 12)
                        .wrapping_add((nes.ppu.bg_next_tile_id as u16) << 4)
                        .wrapping_add(nes.ppu.loopy_v.get_fine_y())
                        .wrapping_add(8));
                },
                7 => {
                    incr_scroll_x(nes);
                },
                _ => {}
            }
        }
    }

    if nes.ppu.scan_cycle == 256 {
        incr_scroll_x(nes);
    }

    if nes.ppu.scan_cycle == 257 {
        transfer_addr_x(nes);
    }

    if nes.ppu.scan_line == 0 && nes.ppu.scan_cycle >= 280 && nes.ppu.scan_cycle < 305 {
        transfer_addr_y(nes);
    }

    // Render background
    if nes.ppu.scan_line < 240 && nes.ppu.scan_cycle < 256 {
        if nes.ppu.reg_mask.render_bg_enabled() {
            let bit_mux = 0x8000 >> nes.ppu.fine_x;
            let mut pixel = (nes.ppu.bg_shift_pat_lo & bit_mux != 0) as u8;
            pixel |= ((nes.ppu.bg_shift_pat_hi & bit_mux != 0) as u8) << 1;
            let mut palette = (nes.ppu.bg_shift_att_lo & bit_mux != 0) as u8;
            palette |= ((nes.ppu.bg_shift_att_hi & bit_mux != 0) as u8) << 1;
    
            let rgb = PALETTE_TO_RGB[(read(nes, 0x3F00 + ((palette << 2) + pixel) as u16) & 0x3F) as usize];
    
            nes.submit_draw_event(DrawEvent {
                position: ((nes.ppu.scan_cycle as u8).wrapping_sub(1), nes.ppu.scan_line as u8), rgb
            });
        }
    }
    */
    
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
            nes.ppu.loopy_t.set_nametable_x(nes.ppu.reg_control.get_name_x());
            nes.ppu.loopy_t.set_nametable_y(nes.ppu.reg_control.get_name_y());
        },
        PPUMASK => {
            nes.ppu.reg_mask.reg = data;
        },
        PPUSCROLL => {
            if nes.ppu.reg_addr_data.latch {
                nes.ppu.fine_x = data & 0x07;
                nes.ppu.loopy_t.set_coarse_x(data >> 3);
            } else {
                nes.ppu.loopy_t.set_fine_y(data & 0x07);
                nes.ppu.loopy_t.set_coarse_x(data >> 3);
            }
            nes.ppu.reg_addr_data.flip_latch();
        },
        OAMADDR => {

        },
        OAMDATA => {

        },
        PPUADDR => {
            ppu::regaddrdata::write_addr(nes, data);
        },
        PPUDATA => {
            ppu::regaddrdata::write_data(nes, data);
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

// scroll helper methods
fn incr_scroll_x(nes: &mut Nes) {
    if nes.ppu.reg_mask.render_bg_enabled() || nes.ppu.reg_mask.render_spr_enabled() {
        if nes.ppu.loopy_v.get_coarse_x() == 31 {
            nes.ppu.loopy_v.set_coarse_x(0);
            nes.ppu.loopy_v.set_nametable_x(!nes.ppu.loopy_v.get_nametable_x());
        } else {
            nes.ppu.loopy_v.set_coarse_x(nes.ppu.loopy_v.get_coarse_x().wrapping_add(1) as u8);
        }
    }
}

fn incr_scroll_y(nes: &mut Nes) {
    if nes.ppu.reg_mask.render_bg_enabled() || nes.ppu.reg_mask.render_spr_enabled() {
        if nes.ppu.loopy_v.get_fine_y() < 7 {
            nes.ppu.loopy_v.set_fine_y(nes.ppu.loopy_v.get_fine_y().wrapping_add(1) as u8);
        } else {
            nes.ppu.loopy_v.set_fine_y(0);
            if nes.ppu.loopy_v.get_coarse_y() == 29 {
                nes.ppu.loopy_v.set_nametable_y(!nes.ppu.loopy_v.get_nametable_y());
            } else if nes.ppu.loopy_v.get_coarse_y() == 31 {
                nes.ppu.loopy_v.set_coarse_y(0);
            } else {
                nes.ppu.loopy_v.set_coarse_y(nes.ppu.loopy_v.get_coarse_y().wrapping_add(1) as u8);
            }
        }
    }
}

fn transfer_addr_x(nes: &mut Nes) {
    if nes.ppu.reg_mask.render_bg_enabled() || nes.ppu.reg_mask.render_spr_enabled() {
        nes.ppu.loopy_v.set_coarse_x(nes.ppu.loopy_t.get_coarse_x() as u8);
        nes.ppu.loopy_v.set_nametable_x(nes.ppu.loopy_t.get_nametable_x());
    }
}

fn transfer_addr_y(nes: &mut Nes) {
    if nes.ppu.reg_mask.render_bg_enabled() || nes.ppu.reg_mask.render_spr_enabled() {
        nes.ppu.loopy_v.set_coarse_y(nes.ppu.loopy_t.get_coarse_y() as u8);
        nes.ppu.loopy_v.set_nametable_y(nes.ppu.loopy_t.get_nametable_y());
        nes.ppu.loopy_v.set_fine_y(nes.ppu.loopy_t.get_fine_y() as u8);
    }
}

fn load_bg_shifters(nes: &mut Nes) {
    nes.ppu.bg_shift_pat_lo = (nes.ppu.bg_shift_pat_lo & 0xff00) | nes.ppu.bg_next_tile_lsb as u16;
    nes.ppu.bg_shift_pat_hi = (nes.ppu.bg_shift_pat_hi & 0xff00) | nes.ppu.bg_next_tile_msb as u16;
    nes.ppu.bg_shift_att_hi = (nes.ppu.bg_shift_att_hi & 0xff00) | if nes.ppu.bg_next_tile_attr & 0b01 != 0 { 0xff } else { 0x00 };
    nes.ppu.bg_shift_att_hi = (nes.ppu.bg_shift_att_lo & 0xff00) | if nes.ppu.bg_next_tile_attr & 0b10 != 0 { 0xff } else { 0x00 };
}

fn update_shifters(nes: &mut Nes) {
    if nes.ppu.reg_mask.render_bg_enabled() {
        nes.ppu.bg_shift_pat_lo <<= 1;
        nes.ppu.bg_shift_pat_hi <<= 1;
        nes.ppu.bg_shift_att_lo <<= 1;
        nes.ppu.bg_shift_att_hi <<= 1;
    }
}