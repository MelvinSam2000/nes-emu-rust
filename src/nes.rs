use crate::cpu::Cpu;
use crate::buscpu::BusCpu;
use crate::ram::Ram;
use crate::cartprg::CartPrg;
use crate::ppu::Ppu;

pub struct Nes {
    cpu: Cpu
}

impl Nes {
 
    // PUBLIC METHODS

    pub fn new() -> Self {
        
        // Create devices
        let ram = Ram::new();
        let cartprg = CartPrg::new();
        let ppu = Ppu::new();
        let buscpu = BusCpu::new(ram, cartprg, ppu);
        let cpu = Cpu::new(buscpu);

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

}