use crate::nes::Nes;
use crate::cpu::cpu;
use crate::cpu::cpu::CpuFlag;
use crate::cpu::addressing;

pub fn adc(nes: &mut Nes) {

    let mut temp: u16 = nes.cpu.ac.wrapping_add(nes.cpu.data) as u16;
    temp = temp.wrapping_add(cpu::get_flag(nes, CpuFlag::C) as u16);

    cpu::set_flag(nes, CpuFlag::C, temp > 255);
    cpu::set_flag(nes, CpuFlag::Z, (temp & 0x00ff) == 0);
    cpu::set_flag(nes, CpuFlag::V, (!(nes.cpu.ac as u16 ^ nes.cpu.data as u16) & (nes.cpu.ac as u16 ^ temp)) & 0x0080 != 0);
    cpu::set_flag(nes, CpuFlag::N, temp & 0x80 != 0);
    
    nes.cpu.ac = temp as u8;
    nes.cpu.cycles += 1;
}

pub fn and(nes: &mut Nes) {
    nes.cpu.ac &= nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0x00);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x80 != 0);
}

pub fn asl(nes: &mut Nes) {
    let tmp = (nes.cpu.data as u16) << 1;
    
    cpu::set_flag(nes, CpuFlag::C, tmp & 0xff00 != 0);
    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8);
    }
}

pub fn bcc(nes: &mut Nes) {
    if !cpu::get_flag(nes, CpuFlag::C) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
}

pub fn bcs(nes: &mut Nes) {
    if cpu::get_flag(nes, CpuFlag::C) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
}

pub fn beq(nes: &mut Nes) {
    if cpu::get_flag(nes, CpuFlag::Z) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
}

pub fn bit(nes: &mut Nes) {
    let tmp = nes.cpu.ac & nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0x00);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.data & (1 << 7) != 0);
    cpu::set_flag(nes, CpuFlag::V, nes.cpu.data & (1 << 6) != 0);
}

pub fn bmi(nes: &mut Nes) {
    if cpu::get_flag(nes, CpuFlag::N) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
}

pub fn bne(nes: &mut Nes) {
    if !cpu::get_flag(nes, CpuFlag::Z) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
}

pub fn bpl(nes: &mut Nes) {
    if !cpu::get_flag(nes, CpuFlag::N) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
}

pub fn brk(nes: &mut Nes) {
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);

    cpu::set_flag(nes, CpuFlag::I, true);
    cpu::write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), (nes.cpu.pc >> 8) as u8);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    cpu::write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), nes.cpu.pc as u8);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);

    cpu::set_flag(nes, CpuFlag::B, true);
    cpu::write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), nes.cpu.status);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    cpu::set_flag(nes, CpuFlag::B, false);

    nes.cpu.pc = cpu::read(nes, 0xfffe) as u16 | ((cpu::read(nes, 0xffff) as u16) << 8);
}

pub fn bvc(nes: &mut Nes) {
    if !cpu::get_flag(nes, CpuFlag::V) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
}

pub fn bvs(nes: &mut Nes) {
    if cpu::get_flag(nes, CpuFlag::V) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
}

pub fn clc(nes: &mut Nes) {
    cpu::set_flag(nes, CpuFlag::C, false);
}

pub fn cld(nes: &mut Nes) {
    cpu::set_flag(nes, CpuFlag::D, false);
}

pub fn cli(nes: &mut Nes) {
    cpu::set_flag(nes, CpuFlag::I, false);
}

pub fn clv(nes: &mut Nes) {
    cpu::set_flag(nes, CpuFlag::V, false);
}

pub fn cmp(nes: &mut Nes) {
    let tmp = (nes.cpu.ac as u16).wrapping_sub(nes.cpu.data as u16);
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.ac >= nes.cpu.data);
    cpu::set_flag(nes, CpuFlag::Z, (tmp & 0x00ff) == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
    nes.cpu.cycles += 1;
}

pub fn cpx(nes: &mut Nes) {
    let tmp = (nes.cpu.x as u16).wrapping_sub(nes.cpu.data as u16);
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.x >= nes.cpu.data);
    cpu::set_flag(nes, CpuFlag::Z, (tmp & 0x00ff) == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
}

pub fn cpy(nes: &mut Nes) {
    let tmp = (nes.cpu.y as u16).wrapping_sub(nes.cpu.data as u16);
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.y >= nes.cpu.data);
    cpu::set_flag(nes, CpuFlag::Z, (tmp & 0x00ff) == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
}

pub fn dec(nes: &mut Nes) {
    let tmp = nes.cpu.data.wrapping_sub(1);
    cpu::write(nes, nes.cpu.addr, tmp);
    cpu::set_flag(nes, CpuFlag::Z, tmp == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
}

pub fn dex(nes: &mut Nes) {
    nes.cpu.x = nes.cpu.x.wrapping_sub(1);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
}

pub fn dey(nes: &mut Nes) {
    nes.cpu.y = nes.cpu.y.wrapping_sub(1);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.y == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.y & 0x0080 != 0);
}

pub fn eor(nes: &mut Nes) {
    nes.cpu.ac ^= nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
}

pub fn inc(nes: &mut Nes) {
    let tmp = nes.cpu.data.wrapping_add(1);
    cpu::write(nes, nes.cpu.addr, tmp);
    cpu::set_flag(nes, CpuFlag::Z, tmp == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
}

pub fn inx(nes: &mut Nes) {
    nes.cpu.x = nes.cpu.x.wrapping_add(1);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
}

pub fn iny(nes: &mut Nes) {
    nes.cpu.y = nes.cpu.y.wrapping_add(1);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.y == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.y & 0x0080 != 0);
}

pub fn jmp(nes: &mut Nes) {
    nes.cpu.pc = nes.cpu.addr;
}

pub fn jsr(nes: &mut Nes) {
    nes.cpu.pc = nes.cpu.pc.wrapping_sub(1) as u16;
    cpu::write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), ((nes.cpu.pc >> 8) & 0x00ff) as u8);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    cpu::write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), nes.cpu.pc as u8);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    nes.cpu.pc = nes.cpu.addr;
}

pub fn lda(nes: &mut Nes) {
    nes.cpu.ac = nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
}

pub fn ldx(nes: &mut Nes) {
    nes.cpu.x = nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
}

pub fn ldy(nes: &mut Nes) {
    nes.cpu.y = nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.y == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.y & 0x0080 != 0);
}

pub fn lsr(nes: &mut Nes) {

    cpu::set_flag(nes, CpuFlag::C, nes.cpu.data & 0x0001 != 0);

    let tmp = (nes.cpu.data as u16) >> 1;
    
    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8);
    }
}

pub fn nop(_nes: &mut Nes) {
    return;
}

pub fn ora(nes: &mut Nes) {
    nes.cpu.ac |= nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0x00);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x80 != 0);
}

pub fn pha(nes: &mut Nes) {
    cpu::write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), nes.cpu.ac);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
}

pub fn php(nes: &mut Nes) {
    cpu::write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), nes.cpu.status | CpuFlag::B as u8);
    cpu::set_flag(nes, CpuFlag::B, false);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
}

pub fn pla(nes: &mut Nes) {
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.ac = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100));
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0x00);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x80 != 0);
}

pub fn plp(nes: &mut Nes) {
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.status = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100));
    cpu::set_flag(nes, CpuFlag::B, false);
    cpu::set_flag(nes, CpuFlag::U, true);
}

pub fn rol(nes: &mut Nes) {
    let mut tmp = (nes.cpu.data as u16) << 1;
    if cpu::get_flag(nes, CpuFlag::C) {
        tmp |= 0x0001;
    }
    
    cpu::set_flag(nes, CpuFlag::C, tmp & 0xff00 != 0);
    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8);
    }
}

pub fn ror(nes: &mut Nes) {
    let mut tmp = (nes.cpu.data as u16) >> 1;
    if cpu::get_flag(nes, CpuFlag::C) {
        tmp |= 0x0080;
    }
    
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.data & 0x0001 != 0);
    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8);
    }
}

pub fn rti(nes: &mut Nes) {
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.status = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100));
    cpu::set_flag(nes, CpuFlag::U, true);
    cpu::set_flag(nes, CpuFlag::B, false);
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.pc = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100)) as u16;
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.pc |= (cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100)) as u16) << 8;
}

pub fn rts(nes: &mut Nes) {
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.pc = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100)) as u16;
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.pc |= (cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100)) as u16) << 8;
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
}

pub fn sbc(nes: &mut Nes) {
    let temp1: u16 = nes.cpu.data as u16 ^ 0x00ff;
    let mut temp2: u16 = (nes.cpu.ac as u16).wrapping_add(temp1);
    if cpu::get_flag(nes, CpuFlag::C) {
        temp2 = temp2.wrapping_add(1);
    }

    cpu::set_flag(nes, CpuFlag::C, temp2 & 0xff00 != 0);
    cpu::set_flag(nes, CpuFlag::Z, temp2 & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::V, (temp2 ^ nes.cpu.ac as u16) & (temp1 ^ temp2) & 0x0080 != 0);
    cpu::set_flag(nes, CpuFlag::N, temp2 & 0x0080 != 0);
    
    nes.cpu.ac = temp2 as u8;
    nes.cpu.cycles += 1;
}

pub fn sec(nes: &mut Nes) {
    cpu::set_flag(nes, CpuFlag::C, true);
}

pub fn sed(nes: &mut Nes) {
    cpu::set_flag(nes, CpuFlag::D, true);
}

pub fn sei(nes: &mut Nes) {
    cpu::set_flag(nes, CpuFlag::I, true);
}

pub fn sta(nes: &mut Nes) {
    cpu::write(nes, nes.cpu.addr, nes.cpu.ac);
}

pub fn stx(nes: &mut Nes) {
    cpu::write(nes, nes.cpu.addr, nes.cpu.x);
}

pub fn sty(nes: &mut Nes) {
    cpu::write(nes, nes.cpu.addr, nes.cpu.y);
}

pub fn tax(nes: &mut Nes) {
    nes.cpu.x = nes.cpu.ac;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
}

pub fn tay(nes: &mut Nes) {
    nes.cpu.y = nes.cpu.ac;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.y == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.y & 0x0080 != 0);
}

pub fn tsx(nes: &mut Nes) {
    nes.cpu.x = nes.cpu.sp;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
}

pub fn txa(nes: &mut Nes) {
    nes.cpu.ac = nes.cpu.x;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
}

pub fn txs(nes: &mut Nes) {
    nes.cpu.sp = nes.cpu.x;
}

pub fn tya(nes: &mut Nes) {
    nes.cpu.ac = nes.cpu.y;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
}

pub fn xxx(_nes: &mut Nes) {
    return;
}

// UNOFFICIAL OPCODES:

pub fn dcp(nes: &mut Nes) {
    nes.cpu.data = nes.cpu.data.wrapping_sub(1);
    cpu::write(nes, nes.cpu.addr, nes.cpu.data);
    if nes.cpu.ac >= nes.cpu.data {
        cpu::set_flag(nes, CpuFlag::C, true);
    }
    let tmp = nes.cpu.ac.wrapping_sub(nes.cpu.data);
    cpu::set_flag(nes, CpuFlag::Z, tmp == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
}

pub fn dop(nes: &mut Nes) {
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
}

pub fn isb(nes: &mut Nes) {
    let tmp = nes.cpu.data.wrapping_add(1);
    cpu::write(nes, nes.cpu.addr, tmp);
    nes.cpu.ac = nes.cpu.ac.wrapping_sub(tmp);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
}

pub fn lax(nes: &mut Nes) {
    nes.cpu.ac = nes.cpu.data;
    nes.cpu.x = nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.data == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.data & 0x0080 != 0);
}

pub fn rla(nes: &mut Nes) {
    let mut tmp = (nes.cpu.data as u16) << 1;
    if cpu::get_flag(nes, CpuFlag::C) {
        tmp |= 0x0001;
    }
    
    cpu::set_flag(nes, CpuFlag::C, tmp & 0xff00 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8);
    }

    nes.cpu.ac &= tmp as u8;
    
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
}

pub fn rra(nes: &mut Nes) {
    let mut tmp = (nes.cpu.data as u16) >> 1;
    if cpu::get_flag(nes, CpuFlag::C) {
        tmp |= 0x0080;
    }
    
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.data & 0x0001 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8);
    }

    // ADD
    nes.cpu.ac |= tmp as u8;
    
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
}

pub fn sax(nes: &mut Nes) {
    let val = nes.cpu.ac & nes.cpu.x;
    cpu::write(nes, nes.cpu.addr, val);
}

pub fn slo(nes: &mut Nes) {
    let tmp = (nes.cpu.data as u16) << 1;
    
    cpu::set_flag(nes, CpuFlag::C, tmp & 0xff00 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8);
    }

    nes.cpu.ac |= tmp as u8;
    
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
}

pub fn sre(nes: &mut Nes) {
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.data & 0x0001 != 0);

    let tmp = (nes.cpu.data as u16) >> 1;

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8);
    }

    nes.cpu.ac ^= tmp as u8;

    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);

}

pub fn top(nes: &mut Nes) {
    nes.cpu.pc = nes.cpu.pc.wrapping_add(2);
}

