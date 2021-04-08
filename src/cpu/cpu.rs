use crate::buscpu::*;
use crate::cpu::decode::*;

pub struct Cpu {
    // devices
    pub bus: BusCpu,
    
    // registers
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub status: u8,

    // helper variables
    pub cycles: u8,
    pub addr_abs: u16,
    pub addr_rel: u16,
    pub addr_mode: usize,
    pub data: u8,
}

pub enum CpuFlag {
    C = (1 << 0),	// Carry Bit
    Z = (1 << 1),	// Zero
    I = (1 << 2),	// Disable Interrupts
    D = (1 << 3),	// Decimal Mode
    B = (1 << 4),	// Break
    U = (1 << 5),	// Unused
    V = (1 << 6),	// Overflow
    N = (1 << 7),	// Negative
}

impl Cpu {
    pub fn new(bus: BusCpu) -> Self {
        return Self {
            bus,
            
            pc: 0x0000,
            ac: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0x00,
            status: 0x00,

            cycles: 0,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            data: 0x00,
        }
    }

    // EXTERNAL METHODS

    pub fn reset(&mut self) {
        self.ac = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xfd;
        self.status = 0x00;
        self.pc = self.fetch_word(0xfffc);
        self.cycles = 8;
    }

    pub fn clock(&mut self) {
        if self.cycles > 0 {
            self.cycles -= 1;
            return;
        }

        // fetch
        let opcode = self.read(self.pc);
        self.pc += 1;
        // decode
        let decoded = Cpu::decode(opcode);
        self.cycles = decoded.cycles;
        // execute
        self.addr_mode = decoded.addr_mode as usize;
        (decoded.addr_mode)(self);
        (decoded.instruction)(self);
    }

    pub fn irq(&mut self) {
        
        if self.get_flag(CpuFlag::I) {
            return;
        }
        self.write(self.sp as u16 + 0x100, ((self.pc >> 8) & 0x00ff) as u8);
        self.sp -= 1;
        self.write(self.sp as u16 + 0x100, self.pc as u8);
        self.sp -= 1;
        self.write(self.sp as u16 + 0x0100, self.status);
        self.sp -= 1;
        
        self.set_flag(CpuFlag::B, false);
        self.set_flag(CpuFlag::I, true);

        self.pc = self.fetch_word(0xfffe);

        self.cycles = 7;

    }

    pub fn nmi(&mut self) {

        self.write(self.sp as u16 + 0x100, ((self.pc >> 8) & 0x00ff) as u8);
        self.sp -= 1;
        self.write(self.sp as u16 + 0x100, self.pc as u8);
        self.sp -= 1;
        self.write(self.sp as u16 + 0x0100, self.status);
        self.sp -= 1;
        
        self.set_flag(CpuFlag::B, false);
        self.set_flag(CpuFlag::I, true);

        self.pc = self.fetch_word(0xfffa);

        self.cycles = 8;
    }

    // HELPER METHODS

    pub fn read(&mut self, addr: u16) -> u8 {
        return self.bus.read(addr);
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data);
    }

    pub fn set_flag(&mut self, flag: CpuFlag, val: bool) {
        if val {
            self.status |= flag as u8;
        } else {
            self.status &= !(flag as u8);
        }
    }

    pub fn get_flag(&self, flag: CpuFlag) -> bool {
        return flag as u8 & self.status != 0x00;
    }

    pub fn fetch_word(&mut self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr + 1) as u16;
        return hi << 8 | lo;
    }

    pub fn pc_fetch_byte(&mut self) -> u8 {
        let data = self.read(self.pc);
        self.pc += 1;
        return data;
    }

    pub fn pc_fetch_word(&mut self) -> u16 {
        let data = self.fetch_word(self.pc);
        self.pc += 2;
        return data;
    }
}