extern crate wasm_bindgen;
extern crate console_error_panic_hook;

use nes::nes::Nes;
use nes::joypad::Button;
use nes::joypad::Button::*;
use nes::apu::apu::ApuUpdateCode;

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

    // Audio
    pub fn apu_check_updated(&mut self) -> u8 {
        let out = self.nes.apu.updated;
        self.nes.apu.updated = ApuUpdateCode::NONE;
        return out as u8;
    }


    pub fn get_apu_config(&mut self) -> ApuConfig {
        return ApuConfig {
            en_pulse1: self.nes.apu.en_pulse1,
            en_pulse2: self.nes.apu.en_pulse2,
            en_triangle: self.nes.apu.en_triangle,
            pulse1_volume: self.nes.apu.pulse1.volume,
            pulse1_dutycycle: self.nes.apu.pulse1.dutycycle,
            pulse1_frequency: self.nes.apu.pulse1.get_frequency(),
            pulse2_volume: self.nes.apu.pulse2.volume,
            pulse2_dutycycle: self.nes.apu.pulse2.dutycycle,
            pulse2_frequency: self.nes.apu.pulse2.get_frequency(),
            triangle_muted: self.nes.apu.triangle.muted,
            triangle_frequency: self.nes.apu.triangle.get_frequency(),
        };
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

#[wasm_bindgen]
pub struct ApuConfig {
    pub en_pulse1: bool,
    pub en_pulse2: bool,
    pub en_triangle: bool,
    // pulse 1
    pub pulse1_volume: f32,
    pub pulse1_dutycycle: f32,
    pub pulse1_frequency: u16,
    // pulse 2
    pub pulse2_volume: f32,
    pub pulse2_dutycycle: f32,
    pub pulse2_frequency: u16,
    // triangle
    pub triangle_muted: bool,
    pub triangle_frequency: u16,
}