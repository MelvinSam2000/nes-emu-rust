extern crate wasm_bindgen;
extern crate console_error_panic_hook;

use nes::nes::Nes;
use nes::joypad::Button;
use nes::joypad::Button::*;

use wasm_bindgen::prelude::*;
use std::panic;

#[wasm_bindgen]
pub struct WasmNes {
    nes: Nes
}

#[wasm_bindgen]
impl WasmNes {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {

        panic::set_hook(Box::new(console_error_panic_hook::hook));
        
        let nes = Nes::new();
        
        return Self {
            nes,
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