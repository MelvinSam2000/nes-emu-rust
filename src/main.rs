mod nes;
mod buscpu;
mod cpu;
mod ram;
mod cartprg;
mod ppu;

use nes::Nes;

fn main() {

    let mut nes = Nes::new();
    nes.reset();
}
