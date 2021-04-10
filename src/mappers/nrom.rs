use crate::mappers::mapper::Mapper;

pub struct NRom {
    pub prg_banks: u8,
    pub chr_banks: u8
}

impl Mapper for NRom {

    fn read_prg(&self, addr: u16) -> u16 {
        if 0x8000 <= addr {
            if self.prg_banks == 2 {
               return addr - 0x8000;
            }
            if self.prg_banks == 1 {
                return addr & 0x3fff;
            }
        }
        return addr;
    }

    fn write_prg(&self, addr: u16) -> u16 {
        if 0x8000 <= addr {
            if self.prg_banks == 2 {
               return addr - 0x8000;
            }
            if self.prg_banks == 1 {
                return addr & 0x3fff;
            }
        }
        return addr;
    }

    fn read_chr(&self, addr: u16) -> u16 {
        return addr;
    }

    fn write_chr(&self, addr: u16) -> u16 {
        return addr;
    }

}