use std::fs::read;

use crate::cpu::cpu::Cpu;
use crate::mem::buscpu::BusCpu;
use crate::cartprg::CartPrg;
use crate::ppu::Ppu;
use crate::mappers::mapper::Mapper;

pub struct Nes {
    cpu: Cpu
}

impl Nes {
 
    // PUBLIC METHODS

    pub fn new() -> Self {
        
        // Create devices
        let cartprg = CartPrg::new();
        let ppu = Ppu::new();
        let buscpu = BusCpu::new(cartprg, ppu);
        let cpu = Cpu::new(Box::new(buscpu));

        return Self {
            cpu
        };
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn clock(&mut self) {
        self.cpu.clock();
    }

    pub fn load(&mut self, nes_file_path: String) {
        let file_bytes: Vec<u8> = match read(nes_file_path) {
            Err(e) => vec![],
            Ok(v) => v
        };

        // read file header
        let prg_banks = file_bytes[0x4] as usize;
        let chr_banks = file_bytes[0x5] as usize;
        let trainer_is_present = file_bytes[0x6] & 0x04 != 0;

        // resize memories
        let prg_size = 0x4000*prg_banks;
        let chr_size = 0x2000*chr_banks;
        //self.cpu.bus.cartprg.resize(prg_size);

        // fill memories
        let mut offset = 16;
        if trainer_is_present {
            offset += 512;
        }
        //for i in 0..prg_size as u16 {
        //    self.cpu.bus.cartprg.mem[i as usize] = file_bytes[(offset + i) as usize];
        //}

    }

}