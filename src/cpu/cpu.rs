use crate::nes::Nes;
use crate::cpu::decode;
use crate::cpu::addressing;
use crate::buscpu;

pub struct Cpu {
    
    // registers
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub status: u8,

    // helper variables
    pub cycles: u8,
    pub addr: u16,
    pub addr_mode: usize,
    pub data: u8,
    pub is_imp: bool,

    // debug
    pub debug: bool,
    pub debug_ram: Vec<u8>,
}

pub enum CpuFlag {
    C = 1 << 0,	// Carry Bit
    Z = 1 << 1,	// Zero
    I = 1 << 2,	// Disable Interrupts
    D = 1 << 3,	// Decimal Mode (not used)
    B = 1 << 4,	// Break
    U = 1 << 5,	// Unused (break 2)
    V = 1 << 6,	// Overflow
    N = 1 << 7,	// Negative
}

impl Cpu {

    pub fn new() -> Self {
        return Self {
            pc: 0x0000,
            ac: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0x00,
            status: 0x00,

            cycles: 0,
            addr: 0x0000,
            addr_mode: 0x0000,
            data: 0x00,
            is_imp: false,

            debug: false,
            debug_ram: vec![]
        }
    }
}

pub fn reset(nes: &mut Nes) {
    nes.cpu.ac = 0;
    nes.cpu.x = 0;
    nes.cpu.y = 0;
    nes.cpu.sp = 0xfd;
    nes.cpu.status = 0x00 | CpuFlag::I as u8 | CpuFlag::U as u8;
    nes.cpu.pc = fetch_word(nes, 0xfffc);
    nes.cpu.cycles = 8;
}

pub fn clock(nes: &mut Nes) {
    if nes.cpu.cycles > 0 {
        nes.cpu.cycles -= 1;
        return;
    }

    // fetch
    let opcode = read(nes, nes.cpu.pc);
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);;
    // decode
    let decoded = decode::decode(opcode);
    nes.cpu.cycles = decoded.cycles;
    // execute
    nes.cpu.addr_mode = decoded.addr_mode as usize;
    (decoded.addr_mode)(nes);
    (decoded.instruction)(nes);
}

pub fn irq(nes: &mut Nes) {
    
    if get_flag(nes, CpuFlag::I) {
        return;
    }
    write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), ((nes.cpu.pc >> 8) & 0x00ff) as u8);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), nes.cpu.pc as u8);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), nes.cpu.status);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    
    set_flag(nes, CpuFlag::B, false);
    set_flag(nes, CpuFlag::I, true);

    nes.cpu.pc = fetch_word(nes, 0xfffe);

    nes.cpu.cycles = 7;

}

// For debugging
pub fn step(nes: &mut Nes) -> String {
    
    let lg_pc = nes.cpu.pc;
    let decoded = decode::decode(read(nes, lg_pc));

    let (a, x, y, p, sp) = (nes.cpu.ac, nes.cpu.x, nes.cpu.y, nes.cpu.status, nes.cpu.sp);

    clock(nes);
    while nes.cpu.cycles > 0 {
        clock(nes);
    }
    
    // Format instruction bytes
    let mut inst_bytes = String::from("");
    let mut bytes = [0u8; 3]; 
    if decoded.bytes >= 1 {
        bytes[0] = read(nes, lg_pc);
        inst_bytes.push_str(&format!("{:02X}", bytes[0]));
    }
    if decoded.bytes >= 2 {
        bytes[1] = read(nes, lg_pc.wrapping_add(1));
        inst_bytes.push_str(&format!(" {:02X}", bytes[1]));    
    }
    if decoded.bytes >= 3 {
        bytes[2] = read(nes, lg_pc.wrapping_add(2));
        inst_bytes.push_str(&format!(" {:02X}", bytes[2]));    
    }
    while inst_bytes.len() < 8 {
        inst_bytes.push_str(" ");
    }


    // Format registers and the rest
    let mut asm_instruction = String::from(format!(
        "{:04X}  {:?} \t{} \tA:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
        lg_pc, inst_bytes, decoded.instruction_str, 
        a, x, y, p, sp
    ));
    asm_instruction = asm_instruction.replace("\"", "");
    return asm_instruction;
}

pub fn nmi(nes: &mut Nes) {


    write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), ((nes.cpu.pc >> 8) & 0x00ff) as u8);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), nes.cpu.pc as u8);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), nes.cpu.status);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    
    set_flag(nes, CpuFlag::B, false);
    set_flag(nes, CpuFlag::I, true);

    nes.cpu.pc = fetch_word(nes, 0xfffa);

    nes.cpu.cycles = 8;
    
}

// HELPER METHODS

pub fn read(nes: &mut Nes, addr: u16) -> u8 {
    if nes.cpu.debug {
        return nes.cpu.debug_ram[addr as usize];
    }
    return buscpu::read(nes, addr);
}

pub fn write(nes: &mut Nes, addr: u16, data: u8) {
    if nes.cpu.debug {
        nes.cpu.debug_ram[addr as usize] = data;
        return;
    }
    buscpu::write(nes, addr, data);
}

pub fn set_flag(nes: &mut Nes, flag: CpuFlag, val: bool) {
    if val {
        nes.cpu.status |= flag as u8;
    } else {
        nes.cpu.status &= !(flag as u8);
    }
}

pub fn get_flag(nes: &Nes, flag: CpuFlag) -> bool {
    return flag as u8 & nes.cpu.status != 0x00;
}

pub fn fetch_word(nes: &mut Nes, addr: u16) -> u16 {
    let lo = read(nes, addr) as u16;
    let hi = read(nes, addr.wrapping_add(1)) as u16;
    return hi << 8 | lo;
}

pub fn pc_fetch_byte(nes: &mut Nes) -> u8 {
    let data = read(nes, nes.cpu.pc);
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
    return data;
}

pub fn pc_fetch_word(nes: &mut Nes) -> u16 {
    let data = fetch_word(nes, nes.cpu.pc);
    nes.cpu.pc = nes.cpu.pc.wrapping_add(2);
    return data;
}

pub fn fetch_data(nes: &mut Nes) {
    if !nes.cpu.is_imp {
        nes.cpu.data = read(nes, nes.cpu.addr);
    }
}