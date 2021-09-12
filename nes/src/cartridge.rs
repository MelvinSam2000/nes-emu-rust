use crate::mappers::mapper::Mapper;
use crate::mappers::nrom::NROM;
use crate::mappers::uxrom::UxROM;
use crate::mappers::cnrom::CNROM;
use crate::mappers::gxrom::GxROM;

pub struct Cartridge {
    pub prgmem: Vec<u8>,
    pub chrmem: Vec<u8>,
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub mapper: Mapper,
    pub mirroring: Mirroring,

    // extra mapper variables
    pub uxrom_banksel: u8,
    pub cnrom_banksel: u8,
    pub gxrom_banksel: (u8, u8),
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

            uxrom_banksel: 0,
            cnrom_banksel: 0,
            gxrom_banksel: (0, 0),
        }
    }

    pub fn load_cartridge(&mut self, hexdump: Vec<u8>) {
        // read file header
        let prg_banks = hexdump[0x4];
        let chr_banks = hexdump[0x5];

        let trainer_is_present = hexdump[0x6] & 0x04 != 0;
        let mirroring = hexdump[0x6] & 0x01 != 0;
        let prg_size: u64 = 0x4000*prg_banks as u64;
        let chr_size: u64 = 0x2000*chr_banks as u64;

        self.mirroring = if mirroring { Mirroring::HORIZONTAL } else { Mirroring::VERTICAL };

        // resize cartridge roms
        self.prg_banks = prg_banks as u8;
        self.chr_banks = chr_banks as u8;
        self.prgmem.resize(prg_size as usize, 0);
        self.chrmem.resize(chr_size as usize, 0);
        if chr_size == 0 {
            self.chrmem.resize(0x2000, 0);
        }

        // choose mapper
        let mapper_id = (hexdump[0x7] & 0xf0) | ((hexdump[0x6] & 0xf0) >> 4);
        self.mapper = match mapper_id {
            0 => NROM,
            2 => UxROM,
            3 => CNROM,
            66 => GxROM,
            _ => panic!("Mapper {} not supported yet...", mapper_id),
        };

        // fill memories
        let mut offset = 16;
        if trainer_is_present {
            offset += 512;
        }
        for i in 0..prg_size as u64 {
            self.prgmem[i as usize] = hexdump[(offset + i) as usize];
        }

        for i in 0..chr_size as u64 {
            self.chrmem[i as usize] = hexdump[(prg_size + offset + i) as usize];
        }
    }

    pub fn prg_read(&mut self, addr: u16) -> u8 {
        return (self.mapper.read_prg)(self, addr);
    }

    pub fn prg_write(&mut self, addr: u16, data: u8) {
        (self.mapper.write_prg)(self, addr, data);
    }

    pub fn chr_read(&mut self, addr: u16) -> u8 {
        return (self.mapper.read_chr)(self, addr);
    }

    pub fn chr_write(&mut self, addr: u16, data: u8) {
        (self.mapper.write_chr)(self, addr, data);
    }
}

