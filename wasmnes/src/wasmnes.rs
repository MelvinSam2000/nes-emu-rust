extern crate wasm_bindgen;
extern crate serde;
extern crate console_error_panic_hook;

use nes::nes::Nes;
use nes::joypad::Button;
use nes::joypad::Button::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use web_sys::*;
use std::panic;

#[wasm_bindgen]
extern "C" {
    fn draw_to_canvas(x: u8, y: u8, r: u8, g: u8, b: u8);
}


#[wasm_bindgen]
pub struct WasmNes {
    nes: Nes
}




#[wasm_bindgen]
impl WasmNes {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {

        panic::set_hook(Box::new(console_error_panic_hook::hook));
        
        let mut nes = Nes::new();
        
        return Self {
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

    // Screen

    pub fn get_pixel_r(&mut self, x: u8, y: u8) -> u8 {
        return self.nes.screen[y as usize][x as usize].0;
    }

    pub fn get_pixel_g(&mut self, x: u8, y: u8) -> u8 {
        return self.nes.screen[y as usize][x as usize].1;
    }

    pub fn get_pixel_b(&mut self, x: u8, y: u8) -> u8 {
        return self.nes.screen[y as usize][x as usize].2;
    }

    // Joypad
    pub fn button_pressed(&mut self, btn_id: u8, pressed: bool) {
        
        if btn_id >= 8 {
            return;
        }

        if pressed {
            self.nes.press_btn(btn_map(btn_id));
        } else {
            self.nes.release_btn(btn_map(btn_id));
        }
    }

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

pub fn draw(_: &mut Nes, x: u8, y: u8, rgb: (u8, u8, u8)) {
    draw_to_canvas(x, y, rgb.0, rgb.1, rgb.2);
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

fn btn_map(btn_id: u8) -> Button {
    match btn_id {
        0 => BTN_A,
        1 => BTN_B,
        2 => SELECT,
        3 => START,
        4 => UP,
        5 => DOWN,
        6 => LEFT,
        7 => RIGHT,
        _ => panic!("Invalid button"),
    }
}