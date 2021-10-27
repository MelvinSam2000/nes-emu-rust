use crate::nes::Nes;
use crate::cpu::addressing as addr;
use crate::cpu::instructions as inst;

pub struct DecodedOpcode {
    pub cycles: u8,
    pub bytes: u8,
    pub addr_mode: fn(&mut Nes),
    pub instruction: fn(&mut Nes),
    pub instruction_str: &'static str,
}

pub fn decode(opcode: u8) -> DecodedOpcode {
    match opcode {

        0x00 => wr(7, 1, addr::imp, inst::brk, "BRK"),
        0x01 => wr(6, 2, addr::idx, inst::ora, "ORA"),
        0x03 => wr(8, 2, addr::idx, inst::slo, "SLO"),
        0x04 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0x05 => wr(3, 2, addr::zpg, inst::ora, "ORA"),
        0x06 => wr(5, 2, addr::zpg, inst::asl, "ASL"),
        0x07 => wr(5, 2, addr::zpg, inst::slo, "SLO"),
        0x08 => wr(3, 1, addr::imp, inst::php, "PHP"),
        0x09 => wr(2, 2, addr::imm, inst::ora, "ORA"),
        0x0a => wr(2, 1, addr::imp, inst::asl, "ASL"),
        0x0c => wr(4, 3, addr::xxx, inst::top, "TOP"),
        0x0d => wr(4, 3, addr::abs, inst::ora, "ORA"),
        0x0e => wr(6, 3, addr::abs, inst::asl, "ASL"),
        0x0f => wr(6, 3, addr::abs, inst::slo, "SLO"),

        0x10 => wr(2, 2, addr::rel, inst::bpl, "BPL"),
        0x11 => wr(5, 2, addr::idy, inst::ora, "ORA"),
        0x13 => wr(8, 2, addr::idy, inst::slo, "SLO"),
        0x14 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0x15 => wr(4, 2, addr::zpx, inst::ora, "ORA"),
        0x16 => wr(6, 2, addr::zpx, inst::asl, "ASL"),
        0x17 => wr(6, 2, addr::zpx, inst::slo, "SLO"),
        0x18 => wr(2, 1, addr::imp, inst::clc, "CLC"),
        0x19 => wr(4, 3, addr::aby, inst::ora, "ORA"),
        0x1a => wr(2, 1, addr::xxx, inst::nop, "NOP"),
        0x1b => wr(7, 3, addr::aby, inst::slo, "SLO"),
        0x1c => wr(4, 3, addr::xxx, inst::top, "TOP"),
        0x1d => wr(4, 3, addr::abx, inst::ora, "ORA"),
        0x1e => wr(7, 3, addr::abx, inst::asl, "ASL"),
        0x1f => wr(7, 3, addr::abx, inst::slo, "SLO"),

        0x20 => wr(6, 3, addr::abs, inst::jsr, "JSR"),
        0x21 => wr(6, 2, addr::idx, inst::and, "AND"),
        0x23 => wr(8, 2, addr::idx, inst::rla, "RLA"),
        0x24 => wr(3, 2, addr::zpg, inst::bit, "BIT"),
        0x25 => wr(3, 2, addr::zpg, inst::and, "AND"),
        0x26 => wr(5, 2, addr::zpg, inst::rol, "ROL"),
        0x27 => wr(5, 2, addr::zpg, inst::rla, "RLA"),
        0x28 => wr(4, 1, addr::imp, inst::plp, "PLP"),
        0x29 => wr(2, 2, addr::imm, inst::and, "AND"),
        0x2a => wr(2, 1, addr::imp, inst::rol, "ROL"),
        0x2c => wr(4, 3, addr::abs, inst::bit, "BIT"),
        0x2d => wr(4, 3, addr::abs, inst::and, "AND"),
        0x2e => wr(6, 3, addr::abs, inst::rol, "ROL"),
        0x2f => wr(6, 3, addr::abs, inst::rla, "RLA"),
        
        0x30 => wr(2, 2, addr::rel, inst::bmi, "BMI"),
        0x31 => wr(5, 2, addr::idy, inst::and, "AND"),
        0x33 => wr(8, 2, addr::idy, inst::rla, "RLA"),
        0x34 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0x35 => wr(4, 2, addr::zpx, inst::and, "AND"),
        0x36 => wr(6, 2, addr::zpx, inst::rol, "ROL"),
        0x37 => wr(6, 2, addr::zpx, inst::rla, "RLA"),
        0x38 => wr(2, 1, addr::imp, inst::sec, "SEC"),
        0x39 => wr(4, 3, addr::aby, inst::and, "AND"),
        0x3a => wr(2, 1, addr::xxx, inst::nop, "NOP"),
        0x3b => wr(7, 3, addr::aby, inst::rla, "RLA"),
        0x3c => wr(4, 3, addr::xxx, inst::top, "TOP"),
        0x3d => wr(4, 3, addr::abx, inst::and, "AND"),
        0x3e => wr(7, 3, addr::abx, inst::rol, "ROL"),
        0x3f => wr(7, 3, addr::abx, inst::rla, "RLA"),
        
        0x40 => wr(6, 1, addr::imp, inst::rti, "RTI"),
        0x41 => wr(6, 2, addr::idx, inst::eor, "EOR"),
        0x43 => wr(8, 2, addr::idx, inst::sre, "SRE"),
        0x44 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0x45 => wr(3, 2, addr::zpg, inst::eor, "EOR"),
        0x46 => wr(5, 2, addr::zpg, inst::lsr, "LSR"),
        0x47 => wr(5, 2, addr::zpg, inst::sre, "SRE"),
        0x48 => wr(3, 1, addr::imp, inst::pha, "PHA"),
        0x49 => wr(2, 2, addr::imm, inst::eor, "EOR"),
        0x4a => wr(2, 1, addr::imp, inst::lsr, "LSR"),
        0x4c => wr(3, 3, addr::abs, inst::jmp, "JMP"),
        0x4d => wr(4, 3, addr::abs, inst::eor, "EOR"),
        0x4e => wr(6, 3, addr::abs, inst::lsr, "LSR"),
        0x4f => wr(6, 3, addr::abs, inst::sre, "SRE"),
        
        0x50 => wr(2, 2, addr::rel, inst::bvc, "BVC"),
        0x51 => wr(5, 2, addr::idy, inst::eor, "EOR"),
        0x53 => wr(8, 2, addr::idy, inst::sre, "SRE"),
        0x54 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0x55 => wr(4, 2, addr::zpx, inst::eor, "EOR"),
        0x56 => wr(6, 2, addr::zpx, inst::lsr, "LSR"),
        0x57 => wr(6, 2, addr::zpx, inst::sre, "SRE"),
        0x58 => wr(2, 1, addr::imp, inst::cli, "CLI"),
        0x59 => wr(4, 3, addr::aby, inst::eor, "EOR"),
        0x5a => wr(2, 1, addr::xxx, inst::nop, "NOP"),
        0x5b => wr(7, 3, addr::aby, inst::sre, "SRE"),
        0x5c => wr(4, 3, addr::xxx, inst::top, "TOP"),
        0x5d => wr(4, 3, addr::abx, inst::eor, "EOR"),
        0x5e => wr(7, 3, addr::abx, inst::lsr, "LSR"),
        0x5f => wr(7, 3, addr::abx, inst::sre, "SRE"),

        0x60 => wr(6, 1, addr::imp, inst::rts, "RTS"),
        0x61 => wr(6, 2, addr::idx, inst::adc, "ADC"),
        0x63 => wr(8, 2, addr::idx, inst::rra, "RRA"),
        0x64 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0x65 => wr(3, 2, addr::zpg, inst::adc, "ADC"),
        0x66 => wr(5, 2, addr::zpg, inst::ror, "ROR"),
        0x67 => wr(5, 2, addr::zpg, inst::rra, "RRA"),
        0x68 => wr(4, 1, addr::imp, inst::pla, "PLA"),
        0x69 => wr(2, 2, addr::imm, inst::adc, "ADC"),
        0x6a => wr(2, 1, addr::imp, inst::ror, "ROR"),
        0x6c => wr(6, 3, addr::ind, inst::jmp, "JMP"),
        0x6d => wr(4, 3, addr::abs, inst::adc, "ADC"),
        0x6e => wr(6, 3, addr::abs, inst::ror, "ROR"),
        0x6f => wr(6, 3, addr::abs, inst::rra, "RRA"),

        0x70 => wr(2, 2, addr::rel, inst::bvs, "BVS"),
        0x71 => wr(5, 2, addr::idy, inst::adc, "ADC"),
        0x73 => wr(8, 2, addr::idy, inst::rra, "RRA"),
        0x74 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0x75 => wr(4, 2, addr::zpx, inst::adc, "ADC"),
        0x76 => wr(6, 2, addr::zpx, inst::ror, "ROR"),
        0x77 => wr(6, 2, addr::zpx, inst::rra, "RRA"),
        0x78 => wr(2, 1, addr::imp, inst::sei, "SEI"),
        0x79 => wr(4, 3, addr::aby, inst::adc, "ADC"),
        0x7a => wr(2, 1, addr::xxx, inst::nop, "NOP"),
        0x7b => wr(7, 3, addr::aby, inst::rra, "RRA"),
        0x7c => wr(4, 3, addr::xxx, inst::top, "TOP"),
        0x7d => wr(4, 3, addr::abx, inst::adc, "ADC"),
        0x7e => wr(7, 3, addr::abx, inst::ror, "ROR"),
        0x7f => wr(7, 3, addr::abx, inst::rra, "RRA"),

        0x80 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0x81 => wr(6, 2, addr::idx, inst::sta, "STA"),
        0x82 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0x83 => wr(6, 2, addr::idx, inst::sax, "SAX"),
        0x84 => wr(3, 2, addr::zpg, inst::sty, "STY"),
        0x85 => wr(3, 2, addr::zpg, inst::sta, "STA"),
        0x86 => wr(3, 2, addr::zpg, inst::stx, "STX"),
        0x87 => wr(3, 2, addr::zpg, inst::sax, "SAX"),
        0x88 => wr(2, 1, addr::imp, inst::dey, "DEY"),
        0x89 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0x8a => wr(2, 1, addr::imp, inst::txa, "TXA"),
        0x8c => wr(4, 3, addr::abs, inst::sty, "STY"),
        0x8d => wr(4, 3, addr::abs, inst::sta, "STA"),
        0x8e => wr(4, 3, addr::abs, inst::stx, "STX"),
        0x8f => wr(4, 3, addr::abs, inst::sax, "SAX"),

        0x90 => wr(2, 2, addr::rel, inst::bcc, "BCC"),
        0x91 => wr(6, 2, addr::idy, inst::sta, "STA"),
        0x94 => wr(4, 2, addr::zpx, inst::sty, "STY"),
        0x95 => wr(4, 2, addr::zpx, inst::sta, "STA"),
        0x96 => wr(4, 2, addr::zpy, inst::stx, "STX"),
        0x97 => wr(4, 2, addr::zpy, inst::sax, "SAX"),
        0x98 => wr(2, 1, addr::imp, inst::tya, "TYA"),
        0x99 => wr(5, 3, addr::aby, inst::sta, "STA"),
        0x9a => wr(2, 1, addr::imp, inst::txs, "TXS"),
        0x9d => wr(5, 3, addr::abx, inst::sta, "STA"),

        0xa0 => wr(2, 2, addr::imm, inst::ldy, "LDY"),
        0xa1 => wr(6, 2, addr::idx, inst::lda, "LDA"),
        0xa2 => wr(2, 2, addr::imm, inst::ldx, "LDX"),
        0xa3 => wr(6, 2, addr::idx, inst::lax, "LAX"),
        0xa4 => wr(3, 2, addr::zpg, inst::ldy, "LDY"),
        0xa5 => wr(3, 2, addr::zpg, inst::lda, "LDA"),
        0xa6 => wr(3, 2, addr::zpg, inst::ldx, "LDX"),
        0xa7 => wr(3, 2, addr::zpg, inst::lax, "LAX"),
        0xa8 => wr(2, 1, addr::imp, inst::tay, "TAY"),
        0xa9 => wr(2, 2, addr::imm, inst::lda, "LDA"),
        0xaa => wr(2, 1, addr::imp, inst::tax, "TAX"),
        0xac => wr(4, 3, addr::abs, inst::ldy, "LDY"),
        0xad => wr(4, 3, addr::abs, inst::lda, "LDA"),
        0xae => wr(4, 3, addr::abs, inst::ldx, "LDX"),
        0xaf => wr(4, 3, addr::abs, inst::lax, "LAX"),

        0xb0 => wr(2, 2, addr::rel, inst::bcs, "BCS"),
        0xb1 => wr(5, 2, addr::idy, inst::lda, "LDA"),
        0xb3 => wr(5, 2, addr::idy, inst::lax, "LAX"),
        0xb4 => wr(4, 2, addr::zpx, inst::ldy, "LDY"),
        0xb5 => wr(4, 2, addr::zpx, inst::lda, "LDA"),
        0xb6 => wr(4, 2, addr::zpy, inst::ldx, "LDX"),
        0xb7 => wr(4, 2, addr::zpy, inst::lax, "LAX"),
        0xb8 => wr(2, 1, addr::imp, inst::clv, "CLV"),
        0xb9 => wr(4, 3, addr::aby, inst::lda, "LDA"),
        0xba => wr(2, 1, addr::imp, inst::tsx, "TSX"),
        0xbc => wr(4, 3, addr::abx, inst::ldy, "LDY"),
        0xbd => wr(4, 3, addr::abx, inst::lda, "LDA"),
        0xbe => wr(4, 3, addr::aby, inst::ldx, "LDX"),
        0xbf => wr(4, 3, addr::aby, inst::lax, "LAX"),

        0xc0 => wr(2, 2, addr::imm, inst::cpy, "CPY"),
        0xc1 => wr(6, 2, addr::idx, inst::cmp, "CMP"),
        0xc2 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0xc3 => wr(8, 2, addr::idx, inst::dcp, "DCP"),
        0xc4 => wr(3, 2, addr::zpg, inst::cpy, "CPY"),
        0xc5 => wr(3, 2, addr::zpg, inst::cmp, "CMP"),
        0xc6 => wr(5, 2, addr::zpg, inst::dec, "DEC"),
        0xc7 => wr(5, 2, addr::zpg, inst::dcp, "DCP"),
        0xc8 => wr(2, 1, addr::imp, inst::iny, "INY"),
        0xc9 => wr(2, 2, addr::imm, inst::cmp, "CMP"),
        0xca => wr(2, 1, addr::imp, inst::dex, "DEX"),
        0xcc => wr(4, 3, addr::abs, inst::cpy, "CPY"),
        0xcd => wr(4, 3, addr::abs, inst::cmp, "CMP"),
        0xce => wr(6, 3, addr::abs, inst::dec, "DEC"),
        0xcf => wr(6, 3, addr::abs, inst::dcp, "DCP"),

        0xd0 => wr(2, 2, addr::rel, inst::bne, "BNE"),
        0xd1 => wr(5, 2, addr::idy, inst::cmp, "CMP"),
        0xd3 => wr(8, 2, addr::idy, inst::dcp, "DCP"),
        0xd4 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0xd5 => wr(4, 2, addr::zpx, inst::cmp, "CMP"),
        0xd6 => wr(6, 2, addr::zpx, inst::dec, "DEC"),
        0xd7 => wr(6, 2, addr::zpx, inst::dcp, "DCP"),
        0xd8 => wr(2, 1, addr::imp, inst::cld, "CLD"),
        0xd9 => wr(4, 3, addr::aby, inst::cmp, "CMP"),
        0xda => wr(2, 1, addr::xxx, inst::nop, "NOP"),
        0xdb => wr(7, 3, addr::aby, inst::dcp, "DCP"),
        0xdc => wr(4, 3, addr::xxx, inst::top, "TOP"),
        0xdd => wr(4, 3, addr::abx, inst::cmp, "CMP"),
        0xde => wr(7, 3, addr::abx, inst::dec, "DEC"),
        0xdf => wr(7, 3, addr::abx, inst::dcp, "DCP"),

        0xe0 => wr(2, 2, addr::imm, inst::cpx, "CPX"),
        0xe1 => wr(6, 2, addr::idx, inst::sbc, "SBC"),
        0xe2 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0xe3 => wr(8, 2, addr::idx, inst::isb, "ISB"),
        0xe4 => wr(3, 2, addr::zpg, inst::cpx, "CPX"),
        0xe5 => wr(3, 2, addr::zpg, inst::sbc, "SBC"),
        0xe6 => wr(5, 2, addr::zpg, inst::inc, "INC"),
        0xe7 => wr(5, 2, addr::zpg, inst::isb, "ISB"),
        0xe8 => wr(2, 1, addr::imp, inst::inx, "INX"),
        0xe9 => wr(2, 2, addr::imm, inst::sbc, "SBC"),
        0xea => wr(2, 1, addr::imp, inst::nop, "NOP"),
        0xeb => wr(2, 2, addr::imm, inst::sbc, "SBC"),
        0xec => wr(4, 3, addr::abs, inst::cpx, "CPX"),
        0xed => wr(4, 3, addr::abs, inst::sbc, "SBC"),
        0xee => wr(6, 3, addr::abs, inst::inc, "INC"),
        0xef => wr(6, 3, addr::abs, inst::isb, "ISB"),

        0xf0 => wr(2, 2, addr::rel, inst::beq, "BEQ"),
        0xf1 => wr(5, 2, addr::idy, inst::sbc, "SBC"),
        0xf3 => wr(8, 2, addr::idy, inst::isb, "ISB"),
        0xf4 => wr(3, 2, addr::xxx, inst::dop, "DOP"),
        0xf5 => wr(4, 2, addr::zpx, inst::sbc, "SBC"),
        0xf6 => wr(6, 2, addr::zpx, inst::inc, "INC"),
        0xf7 => wr(6, 2, addr::zpx, inst::isb, "ISB"),
        0xf8 => wr(2, 1, addr::imp, inst::sed, "SED"),
        0xf9 => wr(4, 3, addr::aby, inst::sbc, "SBC"),
        0xfa => wr(2, 1, addr::xxx, inst::nop, "NOP"),
        0xfb => wr(7, 3, addr::aby, inst::isb, "ISB"),
        0xfc => wr(4, 3, addr::xxx, inst::top, "TOP"),
        0xfd => wr(4, 3, addr::abx, inst::sbc, "SBC"),
        0xfe => wr(7, 3, addr::abx, inst::inc, "INC"),
        0xff => wr(7, 3, addr::abx, inst::isb, "ISB"),

        _ => wr(1, 0, addr::xxx, inst::xxx, "XXX")
    }
}


// wrapper to remove function overhead from decode table
fn wr(cycles: u8, bytes: u8, addr_mode: fn(&mut Nes), instruction: fn(&mut Nes), instruction_str: &'static str) -> DecodedOpcode {
    DecodedOpcode {
        cycles, bytes, addr_mode, instruction, instruction_str
    }
}