use crate::nes::Nes;
use crate::ppu::ppu;

pub struct RegAddrData {
    pub reg_addr: u8,
    pub reg_data: u8,
    pub buffer: u16,
    pub latch: bool,
}

impl RegAddrData {

    pub fn new() -> Self {
        return Self {
            reg_addr: 0x00,
            reg_data: 0x00,
            buffer: 0x0000,
            latch: false,
        };
    }

    pub fn set_latch(&mut self, val: bool) {
        self.latch = val;
    }

    pub fn flip_latch(&mut self) {
        self.latch = !self.latch;
    }
    
}

pub fn write_addr(nes: &mut Nes, data: u8) {
    let this = &mut nes.ppu.reg_addr_data;
    if !this.latch {
        nes.ppu.loopy_t.reg = (nes.ppu.loopy_t.reg & 0x00ff) | ((data & 0x3f) as u16) << 8; 
    } else {
        nes.ppu.loopy_t.reg = (nes.ppu.loopy_t.reg & 0xff00) | (data as u16);
        nes.ppu.loopy_v.reg = nes.ppu.loopy_t.reg;
    }
    this.latch = !this.latch;
}

pub fn write_data(nes: &mut Nes, data: u8) {
    ppu::write(nes, nes.ppu.loopy_v.reg, data);
    nes.ppu.loopy_v.reg = nes.ppu.loopy_v.reg.wrapping_add( if nes.ppu.reg_control.is_inc_mode() {32} else {1} );
}

pub fn read_data(nes: &mut Nes) -> u8 {

    let mut out = nes.ppu.reg_addr_data.reg_data;
    nes.ppu.reg_addr_data.reg_data = ppu::read(nes, nes.ppu.loopy_v.reg);

    if 0x3f00 <= nes.ppu.loopy_v.reg {
        out = nes.ppu.reg_addr_data.reg_data;
    }

    // Increment addr
    let inc = if nes.ppu.reg_control.is_inc_mode() { 32 } else { 1 };
    nes.ppu.loopy_t.reg = nes.ppu.loopy_t.reg.wrapping_add(inc);
    return out;
}