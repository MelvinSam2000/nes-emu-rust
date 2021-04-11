mod nes;
mod cpu {
    pub mod cpu;
    pub mod instructions;
    pub mod addressing;
    pub mod decode;
}
mod mappers {
    pub mod mapper;
    pub mod nrom;
}
mod tests {
    pub mod cputest;
}
mod cartridge;
mod buscpu;
mod ppu;

use nes::Nes;


fn main() {

    let mut nes = Nes::new();
    nes.load("games/nestest.nes".to_string());
    nes.reset();
    for _i in 0..1000 {
        nes.clock();
    }
    print!("Hello");
}
