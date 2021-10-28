use crate::mappers::mapper::Mapper;
use crate::mappers::mapper::MapperOperations;
use crate::mappers::nrom::NROM;
use crate::mappers::uxrom::UxROM;
use crate::mappers::cnrom::CNROM;
use crate::mappers::gxrom::GxROM;
use crate::nes::Nes;

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
            mapper: Mapper::NROM(NROM),
            mirroring: Mirroring::HORIZONTAL,
        }
    }
}

pub fn load_cartridge(nes: &mut Nes, hexdump: Vec<u8>) {
    // read file header
    let prg_banks = hexdump[0x4];
    let chr_banks = hexdump[0x5];

    let trainer_is_present = hexdump[0x6] & 0x04 != 0;
    let mirroring = hexdump[0x6] & 0x01 != 0;
    let prg_size: u64 = 0x4000*prg_banks as u64;
    let chr_size: u64 = 0x2000*chr_banks as u64;

    nes.cartridge.mirroring = if mirroring { Mirroring::HORIZONTAL } else { Mirroring::VERTICAL };

    // resize cartridge roms
    nes.cartridge.prg_banks = prg_banks as u8;
    nes.cartridge.chr_banks = chr_banks as u8;
    nes.cartridge.prgmem.resize(prg_size as usize, 0);
    nes.cartridge.chrmem.resize(chr_size as usize, 0);
    if chr_size == 0 {
        nes.cartridge.chrmem.resize(0x2000, 0);
    }

    // choose mapper
    let mapper_id = (hexdump[0x7] & 0xf0) | ((hexdump[0x6] & 0xf0) >> 4);
    nes.cartridge.mapper = match mapper_id {
        0 => Mapper::NROM(NROM::new()),
        2 => Mapper::UxROM(UxROM::new()),
        3 => Mapper::CNROM(CNROM::new()),
        66 => Mapper::GxROM(GxROM::new()),
        _ => panic!("Mapper {} not supported yet...", mapper_id),
    };

    // fill memories
    let mut offset = 16;
    if trainer_is_present {
        offset += 512;
    }
    for i in 0..prg_size as u64 {
        nes.cartridge.prgmem[i as usize] = hexdump[(offset + i) as usize];
    }

    for i in 0..chr_size as u64 {
        nes.cartridge.chrmem[i as usize] = hexdump[(prg_size + offset + i) as usize];
    }
}

pub fn prg_read(nes: &mut Nes, addr: u16) -> u8 {
    Mapper::read_prg(&mut nes.cartridge, addr).unwrap_or(0)
}

pub fn prg_write(nes: &mut Nes, addr: u16, data: u8) {
    Mapper::write_prg(&mut nes.cartridge, addr, data).unwrap_or(())
}

pub fn chr_read(nes: &mut Nes, addr: u16) -> u8 {
    Mapper::read_chr(&mut nes.cartridge, addr).unwrap_or(0)
}

pub fn chr_write(nes: &mut Nes, addr: u16, data: u8) {
    Mapper::write_chr(&mut nes.cartridge, addr, data).unwrap_or(())
}

