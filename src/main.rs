mod nes;
mod buscpu;
mod cpu {
    pub mod cpu;
    pub mod instructions;
    pub mod addressing;
    pub mod decode;
}
mod ram;
mod cartprg;
mod ppu;

use nes::Nes;

fn main() {

    let mut nes = Nes::new();
    nes.reset();
    loop {
        nes.clock();
    }
}
