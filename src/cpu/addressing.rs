use crate::nes::Nes;
use crate::cpu::cpu;

pub fn abs(nes: &mut Nes) {
    nes.cpu.addr = cpu::pc_fetch_word(nes);
    nes.cpu.is_imp = false;
}

pub fn abx(nes: &mut Nes) {
    abs(nes);
    nes.cpu.addr = nes.cpu.addr.wrapping_add(nes.cpu.x as u16);
    nes.cpu.is_imp = false;
}

pub fn aby(nes: &mut Nes) {
    abs(nes);
    nes.cpu.addr = nes.cpu.addr.wrapping_add(nes.cpu.y as u16);
    nes.cpu.is_imp = false;
}

pub fn imm(nes: &mut Nes) {
    nes.cpu.addr = nes.cpu.pc;
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
    nes.cpu.is_imp = false;
}

pub fn imp(nes: &mut Nes) {
    nes.cpu.data = nes.cpu.ac;
    nes.cpu.is_imp = true;
}

pub fn ind(nes: &mut Nes) {
    
    let ptr = cpu::pc_fetch_word(nes);
    // emulate page boundary bug or behave normally
    if ptr & 0x00ff == 0x00ff {
        nes.cpu.addr = cpu::read(nes, ptr) as u16;
        nes.cpu.addr |= (cpu::read(nes, ptr & 0xff00) as u16) << 8;
    } else {
        nes.cpu.addr = cpu::fetch_word(nes, ptr);
    }
    nes.cpu.is_imp = false;
}

pub fn idx(nes: &mut Nes) {
    let ptr = cpu::pc_fetch_byte(nes) as u16;
    let lo = cpu::read(nes, ptr.wrapping_add(nes.cpu.x as u16) & 0x00ff) as u16;
    let hi = cpu::read(nes, ptr.wrapping_add(nes.cpu.x as u16).wrapping_add(1) & 0x00ff) as u16;
    nes.cpu.addr = (hi << 8) | lo;
    nes.cpu.is_imp = false;
}

pub fn idy(nes: &mut Nes) {
    let ptr = cpu::pc_fetch_byte(nes) as u16;
    let lo = cpu::read(nes, ptr & 0x00ff) as u16;
    let hi = cpu::read(nes, ptr.wrapping_add(1) & 0x00ff) as u16;
    nes.cpu.addr = (hi << 8) | lo;
    nes.cpu.addr = nes.cpu.addr.wrapping_add(nes.cpu.y as u16);
    nes.cpu.is_imp = false;
}

pub fn rel(nes: &mut Nes) {
    nes.cpu.addr = cpu::pc_fetch_byte(nes) as u16;
    if nes.cpu.addr & 0x0080 != 0 {
        nes.cpu.addr |= 0xff00;
    }
    nes.cpu.is_imp = false;
}

pub fn zpg(nes: &mut Nes) {
    nes.cpu.addr = cpu::pc_fetch_byte(nes) as u16;
    nes.cpu.addr &= 0x00ff;
    nes.cpu.is_imp = false;
}

pub fn zpx(nes: &mut Nes) {
    nes.cpu.addr = (cpu::pc_fetch_byte(nes).wrapping_add(nes.cpu.x)) as u16;
    nes.cpu.addr &= 0x00ff;
    nes.cpu.is_imp = false;
}

pub fn zpy(nes: &mut Nes) {
    nes.cpu.addr = (cpu::pc_fetch_byte(nes).wrapping_add(nes.cpu.y)) as u16;
    nes.cpu.addr &= 0x00ff;
    nes.cpu.is_imp = false;
}

pub fn xxx(_nes: &mut Nes) {
    return;
}
