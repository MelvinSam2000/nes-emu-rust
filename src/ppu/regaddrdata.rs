use crate::nes::Nes;
use crate::ppu::ppu;

pub struct RegAddrData {
    pub addr_buffer: u16,
    pub reg_data: u8,
    pub latch: bool,
}

impl RegAddrData {

    pub fn new() -> Self {
        return Self {
            addr_buffer: 0x0000,
            reg_data: 0x00,
            latch: false,
        };
    }

    pub fn set_latch(&mut self, val: bool) {
        self.latch = val;
    }
    
}

pub fn write_addr(nes: &mut Nes, data: u8) {
    let this = &mut nes.ppu.reg_addr_data;
    if !this.latch {
        this.addr_buffer |= data as u16; 
    } else {
        this.addr_buffer |= (data as u16) << 8;
    }
    this.latch = !this.latch;
}

pub fn read_data(nes: &mut Nes) -> u8 {
    let addr = nes.ppu.reg_addr_data.addr_buffer & 0x3fff;

    let mut out = nes.ppu.reg_addr_data.reg_data;
    if 0x3f00 <= addr && addr <= 0x3fff {
        out = ppu::read(nes, addr);
    }
    nes.ppu.reg_addr_data.reg_data = ppu::read(nes, addr);

    // Increment addr
    let inc = if nes.ppu.reg_control.is_inc_mode() { 32 } else { 1 };
    nes.ppu.reg_addr_data.addr_buffer = nes.ppu.reg_addr_data.addr_buffer.wrapping_add(inc);
    nes.ppu.reg_addr_data.addr_buffer &= 0x3fff;
    return out;
}