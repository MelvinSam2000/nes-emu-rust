use crate::mappers::mapper::Mapper;
use crate::mappers::nrom::NRom;

pub struct Cartridge {
    pub prgmem: Vec<u8>,
    pub chrmem: Vec<u8>,
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub mapper: Box<dyn Mapper>
}

impl Cartridge {

    pub fn new() -> Self {
        return Self {
            prgmem: vec![],
            chrmem: vec![],
            prg_banks: 0,
            chr_banks: 0,
            mapper: Box::new(NRom),
        }
    }

    pub fn prg_read(&mut self, addr: u16) -> u8 {
        let maddr = self.mapper.read_prg(self, addr);
        return self.prgmem[maddr as usize];
    }

    pub fn prg_write(&mut self, addr: u16, data: u8) {
        let maddr = self.mapper.write_prg(self, addr);
        self.prgmem[maddr as usize] = data;
    }

    pub fn chr_read(&mut self, addr: u16) -> u8 {
        let maddr = self.mapper.read_chr(self, addr);
        return self.chrmem[maddr as usize];
    }

    pub fn chr_write(&mut self, addr: u16, data: u8) {
        let maddr = self.mapper.write_chr(self, addr);
        self.prgmem[maddr as usize] = data;
    }
}

