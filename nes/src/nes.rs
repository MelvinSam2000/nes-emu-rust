use std::fs::read;

use crate::cpu::cpu::Cpu;
use crate::cpu::cpu;
use crate::ppu::ppu::Ppu;
use crate::ppu::ppu;
use crate::apu::apu::Apu;
use crate::buscpu::BusCpu;
use crate::busppu::BusPpu;
use crate::cartridge::{self, Cartridge};
use crate::joypad::*;

pub struct Nes {
    // devices
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub apu: Apu,
    pub cartridge: Cartridge,
    pub buscpu: BusCpu,
    pub busppu: BusPpu,
    pub joypad: Joypad,
    // io
    pub screen: [[(u8, u8, u8); 256]; 240],
    pub screen_hook: fn(&mut Nes, u8, u8, (u8, u8, u8)),
    // helper
    pub clock_count: u64,
}

impl Nes {
 
    // PUBLIC METHODS
    pub fn new() -> Self {
        
        // Create devices
        let cartridge = Cartridge::new();
        let cpu = Cpu::new();
        let ppu = Ppu::new();
        let apu = Apu::new();
        let buscpu = BusCpu::new();
        let busppu = BusPpu::new();
        let joypad = Joypad::new();

        // I/O devices
        let screen: [[(u8, u8, u8); 256]; 240] = [[(0, 0, 0); 256]; 240];

        return Self {
            cpu, ppu, apu, cartridge, buscpu, busppu, joypad,
            screen,
            screen_hook: Nes::sdl_draw,
            clock_count: 0,
        };
    }

    pub fn reset(&mut self) {
        cpu::reset(self);
    }

    pub fn clock(&mut self) {
        if self.clock_count % 3 == 0 {
            cpu::clock(self);
        }
        if self.clock_count == 100000u64 {
            //ppu::draw_chr(self, 1);
            //ppu::get_palette_tbl(self);
            self.clock_count = 0;
        }
        ppu::clock(self);
        self.clock_count = self.clock_count.wrapping_add(1);
    }

    pub fn clock_debug(&mut self) {
        if self.clock_count % 3 == 0 {
            let log = cpu::step(self);
            println!("{:?}", log);
        }
        ppu::clock(self);
        self.clock_count = self.clock_count.wrapping_add(1);
    }

    pub fn load(&mut self, rom: Vec<u8>) {
        cartridge::load_cartridge(self, rom);
    }

    pub fn load_file(&mut self, nes_file_path: String) {
        let file_bytes: Vec<u8> = read(nes_file_path).expect("Could not read file!");
        cartridge::load_cartridge(self, file_bytes);
    }

    // for running small programns, not from ines roms
    pub fn load_debug(&mut self, prg: Vec<u8>) {
        self.cpu.debug = true;
        self.cpu.debug_ram = prg;
        self.cpu.debug_ram.resize(0x10000, 0);
    }

    pub fn draw_pixel(&mut self, x: u8, y: u8, rgb: (u8, u8, u8)) {
        (self.screen_hook)(self, x, y, rgb);
    }

    pub fn sdl_draw(&mut self, x: u8, y: u8, rgb: (u8, u8, u8)) {
        self.screen[y as usize][x as usize] = rgb;
    }

    pub fn press_btn(&mut self, btn: Button) {
        self.joypad.press(btn);
    }

    pub fn release_btn(&mut self, btn: Button) {
        self.joypad.release(btn);
    }

}