use crate::mappers::mapper::Mapper;
use crate::mappers::nrom::NROM;

use std::fs::OpenOptions;
use std::io::Write;

pub struct Cartridge {
    pub prgmem: Vec<u8>,
    pub chrmem: Vec<u8>,
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub mapper: Mapper,
    pub mirroring: Mirroring,
}

pub enum Mirroring {
    HORIZONTAL,
    VERTICAL
}

impl Cartridge {

    pub fn new() -> Self {
        return Self {
            prgmem: vec![],
            chrmem: vec![],
            prg_banks: 0,
            chr_banks: 0,
            mapper: NROM,
            mirroring: Mirroring::HORIZONTAL,
        }
    }

    pub fn load_cartridge(&mut self, hexdump: Vec<u8>) {
        // read file header
        let prg_banks = hexdump[0x4] as usize;
        let chr_banks = hexdump[0x5] as usize;
        let trainer_is_present = hexdump[0x6] & 0x04 != 0;
        let mirroring = hexdump[0x6] & 0x01 != 0;
        let prg_size = 0x4000*prg_banks;
        let chr_size = 0x2000*chr_banks;

        self.mirroring = if mirroring { Mirroring::HORIZONTAL } else { Mirroring::VERTICAL };

        // resize cartridge roms
        self.prg_banks = prg_banks as u8;
        self.chr_banks = chr_banks as u8;
        self.prgmem.resize(prg_size, 0);
        self.chrmem.resize(chr_size, 0);

        // fill memories
        let mut offset = 16;
        if trainer_is_present {
            offset += 512;
        }
        for i in 0..prg_size as u16 {
            self.prgmem[i as usize] = hexdump[(offset + i) as usize];
        }

        for i in 0..chr_size as u16 {
            self.chrmem[i as usize] = hexdump[(prg_size as u16 + offset + i) as usize];
        }

        
    }

    pub fn prg_read(&mut self, addr: u16) -> u8 {
        let maddr = (self.mapper.read_prg)(self, addr);
        if maddr > self.prgmem.len() as u16 {
            return 0x00;
        }
        return self.prgmem[maddr as usize];
    }

    pub fn prg_write(&mut self, addr: u16, data: u8) {
        let maddr = (self.mapper.write_prg)(self, addr);
        if maddr > self.prgmem.len() as u16 {
            return;
        }
        self.prgmem[maddr as usize] = data;
    }

    pub fn chr_read(&mut self, addr: u16) -> u8 {
        let maddr = (self.mapper.read_chr)(self, addr);
        if maddr > self.chrmem.len() as u16 {
            return 0x00;
        }
        return self.chrmem[maddr as usize];
    }

    pub fn chr_write(&mut self, addr: u16, data: u8) {
        let maddr = (self.mapper.write_chr)(self, addr);
        if maddr > self.chrmem.len() as u16 {
            return;
        }
        self.chrmem[maddr as usize] = data;
    }
}

