use std::fs::read;

use crate::cpu::cpu::Cpu;
use crate::cpu::cpu;
use crate::ppu::ppu::Ppu;
use crate::ppu::ppu;
use crate::buscpu::BusCpu;
use crate::busppu::BusPpu;
use crate::cartridge::Cartridge;
use crate::joypad::*;
use crate::events::drawevent::DrawEvent;

pub struct Nes {
    // devices
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub cartridge: Cartridge,
    pub buscpu: BusCpu,
    pub busppu: BusPpu,
    pub joypad: Joypad,
    // io
    pub screen: [[(u8, u8, u8); 256]; 240],
    // helper
    pub clock_count: u64,
    pub eventbus: Vec<DrawEvent>,
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
        let joypad = Joypad::new();

        // I/O devices
        let screen: [[(u8, u8, u8); 256]; 240] = [[(0, 0, 0); 256]; 240];

        return Self {
            cpu, ppu, cartridge, buscpu, busppu, joypad,
            screen,
            clock_count: 0,
            eventbus: vec![],
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

    pub fn load(&mut self, nes_file_path: String) {
        let file_bytes: Vec<u8> = match read(nes_file_path) {
            Err(_e) => vec![],
            Ok(v) => v
        };
        self.cartridge.load_cartridge(file_bytes);
    }

    // for running small programns, not from ines roms
    pub fn load_debug(&mut self, prg: Vec<u8>) {
        self.cpu.debug = true;
        self.cpu.debug_ram = prg;
        self.cpu.debug_ram.resize(0x10000, 0);
    }

    pub fn draw_pixel(&mut self, x: u8, y: u8, rgb: (u8, u8, u8)) {
        self.submit_draw_event(DrawEvent { position: (x, y), rgb});
        //self.screen[y as usize][x as usize] = rgb;
    }

    pub fn get_draw_events(&mut self) -> Vec<DrawEvent> {
        if self.eventbus.len() > 0 {
            let out = self.eventbus.to_vec();
            self.eventbus = vec![];
            return out;
        }
        return vec![];
    }

    pub fn submit_draw_event(&mut self, evt: DrawEvent) {
        self.eventbus.push(evt);
    }

    pub fn screen_pixel(&self, i: u8, j: u8) -> (u8, u8, u8) {
        return self.screen[i as usize][j as usize];
    }

    pub fn press_btn(&mut self, btn: Button) {
        self.joypad.press(btn);
    }

    pub fn release_btn(&mut self, btn: Button) {
        self.joypad.release(btn);
    }

}