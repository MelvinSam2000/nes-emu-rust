use crate::mappers::mapper::Mapper;
use crate::mappers::nrom::NRom;

pub struct CartPrg {
    pub mem: Vec<u8>,
    pub mapper: Box<dyn Mapper>
}

impl CartPrg {

    pub fn new() -> Self {
        return Self {
            mem: vec![],
            mapper: Box::new(NRom { prg_banks: 1, chr_banks: 1})
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        self.mem.resize(new_size, 0);
    }

    pub fn read(&self, addr: u16) -> u8 {
        let mapped_addr = self.mapper.read_prg(addr);
        match self.mem.get(mapped_addr as usize) {
            Some(data) => *data,
            None => 0x00
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        let mapped_addr: u16 = self.mapper.write_prg(addr);
        self.mem[mapped_addr as usize] = data;
    }

    pub fn set_mapper(&mut self, mapper: Box<dyn Mapper>) {
        self.mapper = mapper;
    }
}