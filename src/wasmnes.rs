extern crate wasm_bindgen;
extern crate serde;

use wasm_bindgen::prelude::*;
use crate::nes::Nes;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct WasmNes {
    nes: Nes
}

#[wasm_bindgen]
impl WasmNes {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        return WasmNes {
            nes: Nes::new()
        }
    }

    pub fn reset(&mut self) {
        self.nes.reset();
    }

    pub fn load(&mut self, rom: Vec<u8>) {
        self.nes.load(rom);
    }

    pub fn clock(&mut self) {
        self.nes.clock();
    }

    // Encodes rgb data as u32. Given (rr, gg, bb), returns 0x00rrggbb
    pub fn screen_pixel(&mut self, x: u8, y: u8) -> u32 {
        let (r, g, b) = self.nes.screen_pixel(x, y);
        let val = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        return val;
    }

    /*
    pub fn get_screen_ptr(&self) -> *const [(u8, u8, u8); 256] {
        return self.nes.get_screen_ptr();
    }
    */
    pub fn get_cpu_reg(&mut self) -> JsValue {
        
        let regs = CpuRegisters {
            pc: self.nes.cpu.pc,
            ac: self.nes.cpu.ac,
            x: self.nes.cpu.x,
            y: self.nes.cpu.y,
            sp: self.nes.cpu.sp,
            status: self.nes.cpu.status,    
        };

        return JsValue::from_serde(&regs).unwrap();
    }
}

#[derive(Serialize, Deserialize)]
pub struct CpuRegisters {
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub status: u8,
}
