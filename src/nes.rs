use std::fs::read;

use crate::cpu::cpu::Cpu;
use crate::cpu::cpu;
use crate::ppu::Ppu;
use crate::ppu;
use crate::buscpu::BusCpu;
use crate::busppu::BusPpu;
use crate::cartridge::Cartridge;

pub struct Nes {
    // devices
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub cartridge: Cartridge,
    pub buscpu: BusCpu,
    pub busppu: BusPpu,
    // io
    pub screen: [[(u8, u8, u8); 255]; 240],
    // helper
    pub clock_count: u8,
}

impl Nes {
 
    // PUBLIC METHODS

    pub fn new() -> Self {
        
        // Create devices
        let cartridge = Cartridge::new();
        let cpu = Cpu::new();
        let ppu = Ppu::new();
        let buscpu = BusCpu::new();
        let busppu = BusPpu::new();

        // I/O devices
        let screen: [[(u8, u8, u8); 255]; 240] = [[(0, 0, 0); 255]; 240];

        return Self {
            cpu, ppu, cartridge, buscpu, busppu,
            screen,
            clock_count: 0
        };
    }

    pub fn reset(&mut self) {
        cpu::reset(self);
    }

    pub fn clock(&mut self) {
        if self.clock_count % 3 == 0 {
            cpu::clock(self);
        }
        ppu::clock(self);
        self.clock_count = self.clock_count.wrapping_add(1);
    }

    pub fn load(&mut self, nes_file_path: String) {
        let file_bytes: Vec<u8> = match read(nes_file_path) {
            Err(_e) => vec![],
            Ok(v) => v
        };
        // read file header
        let prg_banks = file_bytes[0x4] as usize;
        let chr_banks = file_bytes[0x5] as usize;
        let trainer_is_present = file_bytes[0x6] & 0x04 != 0;
        let prg_size = 0x4000*prg_banks;
        let chr_size = 0x2000*chr_banks;

        // resize cartridge roms
        self.cartridge.prg_banks = prg_banks as u8;
        self.cartridge.chr_banks = chr_banks as u8;
        self.cartridge.prgmem.resize(prg_size, 0);
        self.cartridge.chrmem.resize(chr_size, 0);

        // fill memories
        let mut offset = 16;
        if trainer_is_present {
            offset += 512;
        }
        for i in 0..prg_size as u16 {
            self.cartridge.prgmem[i as usize] = file_bytes[(offset + i) as usize];
        }
        for i in 0..chr_size as u16 {
            self.cartridge.chrmem[i as usize] = file_bytes[(prg_size as u16 + offset + i) as usize];
        }
    }

    pub fn load_debug(&mut self, prg: Vec<u8>) {
        self.cpu.debug = true;
        self.cpu.debug_ram = prg;
        self.cpu.debug_ram.resize(0x10000, 0);
    }

    pub fn screen_pixel(&self, i: u8, j: u8) -> (u8, u8, u8) {
        return self.screen[i as usize][j as usize];
    }

}