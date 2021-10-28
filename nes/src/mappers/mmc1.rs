use bitflags::bitflags;

use crate::cartridge::{Cartridge, Mirroring};

use super::mapper::{Mapper, MapperOperations};

bitflags! {
    struct RegControl: u8 {
        // Mirroring: (0: one-screen, lower bank; 1: one-screen, upper bank, 2: vertical; 3: horizontal)
        const M0 = 1 << 0;
        const M1 = 1 << 1;
        /*
        P0- P1: PRG ROM bank mode (0, 1: switch 32 KB at $8000, ignoring low bit of bank number;
            2: fix first bank at $8000 and switch 16 KB bank at $C000;
            : fix last bank at $C000 and switch 16 KB bank at $8000)
        */
        const P0 = 1 << 2;
        const P1 = 1 << 3;
        // CHR ROM bank mode (0: switch 8 KB at a time; 1: switch two separate 4 KB banks)
        const C = 1 << 4;
    }
}


pub struct MMC1 {
    reg_load: u8,
    load_count: usize,
    reg_control: RegControl,
    mirror_mode: Mirroring,
    wram: Vec<u8>,

    prg_bank_sel_16_lo: u8,
    prg_bank_sel_16_hi: u8,
    prg_bank_sel_32: u8,

    chr_bank_sel_4_lo: u8,
    chr_bank_sel_4_hi: u8,
    chr_bank_sel_8: u8,
}

impl MMC1 {

    pub fn new() -> Self {
        Self {
            reg_load: 0x00,
            load_count: 0,
            reg_control: RegControl::from_bits_truncate(0),
            mirror_mode: Mirroring::HORIZONTAL,
            wram: vec![],

            prg_bank_sel_16_lo: 0x00,
            prg_bank_sel_16_hi: 0x00,
            prg_bank_sel_32: 0x00,

            chr_bank_sel_4_lo: 0x00,
            chr_bank_sel_4_hi: 0x00,
            chr_bank_sel_8: 0x00,
        }
    }

    pub fn reset(&mut self) {
        self.reg_load = 0x00;
        self.load_count = 0;
        self.reg_control.bits = 0x1c;
    }
}

impl MapperOperations for MMC1 {

    fn read_prg(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        if let Mapper::MMC1(mmc1) = &mut cart.mapper {
            let mut mapped_addr = 0u16;
            match addr {
                0x6000..=0x7fff => {
                    return Ok(mmc1.wram[(addr & 0x1fff) as usize])
                },
                0x8000..=0xffff => {
                    if mmc1.reg_control.contains(RegControl::P1) {
                        match addr {
                            0x8000..=0xbfff => {
                                mapped_addr = mmc1.prg_bank_sel_16_lo as u16 * 0x4000 + (addr & 0x3fff);
                            },
                            0xc000..=0xffff => {
                                mapped_addr = mmc1.prg_bank_sel_16_hi as u16 * 0x4000 + (addr & 0x3fff);
                            },
                            _ => { /* Impossible */}
                        }; 
                    } else {
                        mapped_addr = mmc1.prg_bank_sel_32 as u16 * 0x8000 + (addr & 0x7fff);
                    }
                    Ok(cart.prgmem[mapped_addr as usize])
                }
                _ => Err(())
            }
        } else {
            Err(())
        }
    }

    fn write_prg(cart: &mut Cartridge, addr: u16, data: u8) -> Result<(), ()> {
        if let Mapper::MMC1(mmc1) = &mut cart.mapper {
            match addr {
                0x6000..=0x7fff => {
                    mmc1.wram[(addr & 0x1fff) as usize] = data;
                    Ok(())
                },
                0x8000..=0xffff => {
                    if data & 0x80 != 0 {
                        mmc1.reg_load = 0x00;
                        mmc1.load_count = 0;
                        mmc1.reg_control.bits |= 0xc0;
                    } else {
                        mmc1.reg_load >>= 1;
                        mmc1.reg_load |= (data & 0x01) << 4;
                        mmc1.load_count += 1;
                        
                        if mmc1.load_count == 5 {

                            match addr {
                                0x8000..=0x9fff => {
                                    mmc1.reg_control.bits = mmc1.reg_load & 0x1f;
                                    mmc1.mirror_mode = match mmc1.reg_control.bits {
                                        0 => Mirroring::ONESCREEN_LO,
                                        1 => Mirroring::ONESCREEN_HI,
                                        2 => Mirroring::VERTICAL,
                                        3 => Mirroring::HORIZONTAL,
                                        _ => Mirroring::VERTICAL,
                                    }
                                },
                                0xa000..=0xbfff => {
                                    if mmc1.reg_control.contains(RegControl::C) {
                                        mmc1.chr_bank_sel_4_lo = mmc1.reg_load & 0x1f;
                                    } else {
                                        mmc1.chr_bank_sel_8 = mmc1.reg_load & 0x1e;
                                    }
                                },
                                0xc000..=0xdfff => {
                                    if mmc1.reg_control.contains(RegControl::C) {
                                        mmc1.chr_bank_sel_4_hi = mmc1.reg_load & 0x1f;
                                    }
                                },
                                0xe000..=0xffff => {
                                    let prgmode = (mmc1.reg_control.bits >> 2) & 0x03;
                                    match prgmode {
                                        0 | 1 => {
                                            mmc1.prg_bank_sel_32 = (mmc1.reg_load & 0x0e) >> 1;
                                        },
                                        2 => {
                                            mmc1.prg_bank_sel_16_lo = 0;
                                            mmc1.prg_bank_sel_16_hi = mmc1.reg_load & 0x0f;
                                        },
                                        3 => {
                                            mmc1.prg_bank_sel_16_lo = mmc1.reg_load & 0x0f;
                                            mmc1.prg_bank_sel_16_hi = cart.prg_banks - 1;
                                        },
                                        _ => {}
                                    }
                                },
                                _ => {}
                            }

                            mmc1.reg_load = 0x00;
                            mmc1.load_count = 0;
                        }
                    }
                    Ok(())
                },
                _ => Err(())
            }
        } else {
            Err(())
        }
    }

    fn read_chr(cart: &mut Cartridge, addr: u16) -> Result<u8, ()> {
        if let Mapper::MMC1(mmc1) = &mut cart.mapper {
            let mut mapped_addr = 0u16;
            match addr {
                0x0000..=0x1fff => {
                    if cart.chr_banks == 0 {
                        mapped_addr = addr;
                    } else {
                        if mmc1.reg_control.contains(RegControl::C) {
                            match addr {
                                0x8000..=0xbfff => {
                                    mapped_addr = mmc1.chr_bank_sel_4_lo as u16 * 0x1000 + (addr & 0x0fff);
                                },
                                0xc000..=0xffff => {
                                    mapped_addr = mmc1.chr_bank_sel_4_hi as u16 * 0x1000 + (addr & 0x0fff);
                                },
                                _ => { /* Impossible */}
                            }; 
                        } else {
                            mapped_addr = mmc1.chr_bank_sel_8 as u16 * 0x2000 + (addr & 0x1fff);
                        }
                    }
                    Ok(cart.prgmem[mapped_addr as usize])
                }
                _ => Err(())
            }
        } else {
            Err(())
        }
    }

    fn write_chr(cart: &mut Cartridge, addr: u16, data :u8) -> Result<(), ()> {
        if let Mapper::MMC1(mmc1) = &mut cart.mapper {
            if addr < 0x2000 && cart.chr_banks == 0 {
                cart.chrmem[addr as usize] = data;
            }
        }
        Ok(())
    }
}