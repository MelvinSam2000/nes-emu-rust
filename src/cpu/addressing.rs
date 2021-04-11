use crate::nes::Nes;
use crate::cpu::cpu;

pub fn abs(nes: &mut Nes) {
    nes.cpu.addr_abs = cpu::pc_fetch_word(nes);
    nes.cpu.data = cpu::read(nes, nes.cpu.addr_abs);
}

pub fn abx(nes: &mut Nes) {
    abs(nes);
    nes.cpu.addr_abs += nes.cpu.addr_abs.wrapping_add(nes.cpu.x as u16);
    nes.cpu.data = cpu::read(nes, nes.cpu.addr_abs);
}

pub fn aby(nes: &mut Nes) {
    abs(nes);
    nes.cpu.addr_abs = nes.cpu.addr_abs.wrapping_add(nes.cpu.y as u16);
    nes.cpu.data = cpu::read(nes, nes.cpu.addr_abs);
}

pub fn imm(nes: &mut Nes) {
    nes.cpu.addr_abs = nes.cpu.pc;
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
    nes.cpu.data = cpu::read(nes, nes.cpu.addr_abs);
}

pub fn imp(nes: &mut Nes) {
    nes.cpu.data = nes.cpu.ac;
}

pub fn ind(nes: &mut Nes) {
    // apparently there is a bug here
    let ptr = cpu::pc_fetch_word(nes);
    let addr = cpu::fetch_word(nes, ptr);
    nes.cpu.addr_abs = addr;
    nes.cpu.data = cpu::read(nes, nes.cpu.addr_abs);
}

pub fn idx(nes: &mut Nes) {
    let ptr = cpu::pc_fetch_byte(nes) as u16;
    let lo = cpu::read(nes, ptr.wrapping_add(nes.cpu.x as u16).wrapping_add(1) & 0x00ff) as u16;
    let hi = cpu::read(nes, ptr.wrapping_add(nes.cpu.x as u16).wrapping_add(1) & 0x00ff) as u16;
    nes.cpu.addr_abs = (hi << 8) | lo;
    nes.cpu.data = cpu::read(nes, nes.cpu.addr_abs);
}

pub fn idy(nes: &mut Nes) {
    let ptr = cpu::pc_fetch_byte(nes) as u16;
    let lo = cpu::read(nes, ptr.wrapping_add(nes.cpu.y as u16).wrapping_add(1) & 0x00ff) as u16;
    let hi = cpu::read(nes, ptr.wrapping_add(nes.cpu.y as u16).wrapping_add(1) & 0x00ff) as u16;
    nes.cpu.addr_abs = (hi << 8) | lo;
    nes.cpu.data = cpu::read(nes, nes.cpu.addr_abs);
}

pub fn rel(nes: &mut Nes) {
    nes.cpu.addr_rel = cpu::pc_fetch_byte(nes) as u16;
    if nes.cpu.addr_rel & 0x0080 != 0 {
        nes.cpu.addr_rel |= 0xff00;
    }
    nes.cpu.data = cpu::read(nes, nes.cpu.addr_abs);
}

pub fn zpg(nes: &mut Nes) {
    nes.cpu.addr_abs = cpu::pc_fetch_byte(nes) as u16;
    nes.cpu.addr_abs &= 0x00ff;
    nes.cpu.data = cpu::read(nes, nes.cpu.addr_abs);
}

pub fn zpx(nes: &mut Nes) {
    nes.cpu.addr_abs = (cpu::pc_fetch_byte(nes) + nes.cpu.x) as u16;
    nes.cpu.addr_abs &= 0x00ff;
    nes.cpu.data = cpu::read(nes, nes.cpu.addr_abs);
}

pub fn zpy(nes: &mut Nes) {
    nes.cpu.addr_abs = (cpu::pc_fetch_byte(nes) + nes.cpu.y) as u16;
    nes.cpu.addr_abs &= 0x00ff;
    nes.cpu.data = cpu::read(nes, nes.cpu.addr_abs);
}

pub fn xxx(_nes: &mut Nes) {
    return;
}
