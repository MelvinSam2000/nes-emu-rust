use crate::cpu::cpu::Cpu;

pub struct DecodedOpcode {
    pub cycles: u8,
    pub addr_mode: fn(&mut Cpu),
    pub instruction: fn(&mut Cpu),
}

impl Cpu {

    pub fn decode(opcode: u8) -> DecodedOpcode {
        match opcode {

            0x00 => wr(7, Cpu::IMP, Cpu::BRK),
            0x01 => wr(6, Cpu::IDX, Cpu::ORA),
            0x05 => wr(3, Cpu::ZPG, Cpu::ORA),
            0x06 => wr(5, Cpu::ZPG, Cpu::ASL),
            0x08 => wr(3, Cpu::IMP, Cpu::PHP),
            0x09 => wr(2, Cpu::IMM, Cpu::ORA),
            0x0a => wr(2, Cpu::IMP, Cpu::ASL),
            0x0d => wr(4, Cpu::ABS, Cpu::ORA),
            0x0e => wr(6, Cpu::ABS, Cpu::ASL),

            0x10 => wr(2, Cpu::REL, Cpu::BPL),
            0x11 => wr(5, Cpu::IDY, Cpu::ORA),
            0x15 => wr(4, Cpu::ZPX, Cpu::ORA),
            0x16 => wr(6, Cpu::ZPX, Cpu::ASL),
            0x18 => wr(2, Cpu::IMP, Cpu::CLC),
            0x19 => wr(4, Cpu::ABY, Cpu::ORA),
            0x1d => wr(4, Cpu::ABX, Cpu::ORA),
            0x1e => wr(7, Cpu::ABX, Cpu::ASL),

            0x20 => wr(6, Cpu::ABS, Cpu::JSR),
            0x21 => wr(6, Cpu::IDX, Cpu::AND),
            0x24 => wr(3, Cpu::ZPG, Cpu::BIT),
            0x25 => wr(3, Cpu::ZPG, Cpu::AND),
            0x26 => wr(5, Cpu::ZPG, Cpu::ROL),
            0x28 => wr(4, Cpu::IMP, Cpu::PLP),
            0x29 => wr(2, Cpu::IMM, Cpu::AND),
            0x2a => wr(2, Cpu::IMP, Cpu::ROL),
            0x2c => wr(4, Cpu::ABS, Cpu::BIT),
            0x2d => wr(4, Cpu::ABS, Cpu::AND),
            0x2e => wr(6, Cpu::ABS, Cpu::ROL),
            
            0x30 => wr(2, Cpu::REL, Cpu::BMI),
            0x31 => wr(5, Cpu::IDY, Cpu::AND),
            0x35 => wr(4, Cpu::ZPX, Cpu::AND),
            0x36 => wr(6, Cpu::ZPX, Cpu::ROL),
            0x38 => wr(2, Cpu::IMP, Cpu::SEC),
            0x39 => wr(4, Cpu::ABY, Cpu::AND),
            0x3d => wr(4, Cpu::ABX, Cpu::AND),
            0x3e => wr(7, Cpu::ABX, Cpu::ROL),
            
            0x40 => wr(6, Cpu::IMP, Cpu::RTI),
            0x41 => wr(6, Cpu::IDX, Cpu::EOR),
            0x45 => wr(3, Cpu::ZPG, Cpu::EOR),
            0x46 => wr(5, Cpu::ZPG, Cpu::LSR),
            0x48 => wr(3, Cpu::IMP, Cpu::PHA),
            0x49 => wr(2, Cpu::IMM, Cpu::EOR),
            0x4a => wr(2, Cpu::IMP, Cpu::LSR),
            0x4c => wr(3, Cpu::ABS, Cpu::JMP),
            0x4d => wr(4, Cpu::ABS, Cpu::EOR),
            0x4e => wr(6, Cpu::ABS, Cpu::LSR),
            
            0x50 => wr(2, Cpu::REL, Cpu::BVC),
            0x51 => wr(5, Cpu::IDY, Cpu::EOR),
            0x55 => wr(4, Cpu::ZPX, Cpu::EOR),
            0x56 => wr(6, Cpu::ZPX, Cpu::LSR),
            0x58 => wr(2, Cpu::IMP, Cpu::CLI),
            0x59 => wr(4, Cpu::ABY, Cpu::EOR),
            0x5d => wr(4, Cpu::ABX, Cpu::EOR),
            0x5e => wr(7, Cpu::ABX, Cpu::LSR),

            0x60 => wr(6, Cpu::IMP, Cpu::RTS),
            0x61 => wr(6, Cpu::IDX, Cpu::ADC),
            0x65 => wr(3, Cpu::ZPG, Cpu::ADC),
            0x66 => wr(5, Cpu::ZPG, Cpu::ROR),
            0x68 => wr(4, Cpu::IMP, Cpu::PLA),
            0x69 => wr(2, Cpu::IMM, Cpu::ADC),
            0x6a => wr(2, Cpu::IMP, Cpu::ROR),
            0x6c => wr(6, Cpu::IND, Cpu::JMP),
            0x6d => wr(4, Cpu::ABS, Cpu::ADC),
            0x6e => wr(6, Cpu::ABS, Cpu::ROR),

            0x70 => wr(2, Cpu::REL, Cpu::BVS),
            0x71 => wr(5, Cpu::IDY, Cpu::ADC),
            0x75 => wr(4, Cpu::ZPX, Cpu::ADC),
            0x76 => wr(6, Cpu::ZPX, Cpu::ROR),
            0x78 => wr(2, Cpu::IMP, Cpu::SEI),
            0x79 => wr(4, Cpu::ABY, Cpu::ADC),
            0x7d => wr(4, Cpu::ABX, Cpu::ADC),
            0x7e => wr(7, Cpu::ABX, Cpu::ROR),

            0x81 => wr(6, Cpu::IDX, Cpu::STA),
            0x84 => wr(3, Cpu::ZPG, Cpu::STY),
            0x85 => wr(3, Cpu::ZPG, Cpu::STA),
            0x86 => wr(3, Cpu::ZPG, Cpu::STX),
            0x88 => wr(2, Cpu::IMP, Cpu::DEY),
            0x8a => wr(2, Cpu::IMP, Cpu::TXA),
            0x8c => wr(4, Cpu::ABS, Cpu::STY),
            0x8d => wr(4, Cpu::ABS, Cpu::STA),
            0x8e => wr(4, Cpu::ABS, Cpu::STX),

            0x90 => wr(2, Cpu::REL, Cpu::BCC),
            0x91 => wr(6, Cpu::IDY, Cpu::STA),
            0x94 => wr(4, Cpu::ZPX, Cpu::STY),
            0x95 => wr(4, Cpu::ZPX, Cpu::STA),
            0x96 => wr(4, Cpu::ZPY, Cpu::STX),
            0x98 => wr(2, Cpu::IMP, Cpu::TYA),
            0x99 => wr(5, Cpu::ABY, Cpu::STA),
            0x9a => wr(2, Cpu::IMP, Cpu::TXS),
            0x9d => wr(5, Cpu::ABX, Cpu::STA),

            0xa0 => wr(2, Cpu::IMM, Cpu::LDY),
            0xa1 => wr(6, Cpu::IDX, Cpu::LDA),
            0xa2 => wr(2, Cpu::IMM, Cpu::LDX),
            0xa4 => wr(3, Cpu::ZPG, Cpu::LDY),
            0xa5 => wr(3, Cpu::ZPG, Cpu::LDA),
            0xa6 => wr(3, Cpu::ZPG, Cpu::LDX),
            0xa8 => wr(2, Cpu::IMP, Cpu::TAY),
            0xa9 => wr(2, Cpu::IMM, Cpu::LDA),
            0xaa => wr(2, Cpu::IMP, Cpu::TAX),
            0xac => wr(4, Cpu::ABS, Cpu::LDY),
            0xad => wr(4, Cpu::ABS, Cpu::LDA),
            0xae => wr(4, Cpu::ABS, Cpu::LDX),

            0xb0 => wr(2, Cpu::REL, Cpu::BCS),
            0xb1 => wr(5, Cpu::IDY, Cpu::LDA),
            0xb4 => wr(4, Cpu::ZPX, Cpu::LDY),
            0xb5 => wr(4, Cpu::ZPX, Cpu::LDA),
            0xb6 => wr(4, Cpu::ZPY, Cpu::LDX),
            0xb8 => wr(2, Cpu::IMP, Cpu::CLV),
            0xb9 => wr(4, Cpu::ABY, Cpu::LDA),
            0xba => wr(2, Cpu::IMP, Cpu::TSX),
            0xbc => wr(4, Cpu::ABX, Cpu::LDY),
            0xbd => wr(4, Cpu::ABX, Cpu::LDA),
            0xbe => wr(4, Cpu::ABY, Cpu::LDX),

            0xc0 => wr(2, Cpu::IMM, Cpu::CPY),
            0xc1 => wr(6, Cpu::IDX, Cpu::CMP),
            0xc4 => wr(3, Cpu::ZPG, Cpu::CPY),
            0xc5 => wr(3, Cpu::ZPG, Cpu::CMP),
            0xc6 => wr(5, Cpu::ZPG, Cpu::DEC),
            0xc8 => wr(2, Cpu::IMP, Cpu::INY),
            0xc9 => wr(2, Cpu::IMM, Cpu::CMP),
            0xca => wr(2, Cpu::IMP, Cpu::DEX),
            0xcc => wr(4, Cpu::ABS, Cpu::CPY),
            0xcd => wr(4, Cpu::ABS, Cpu::CMP),
            0xce => wr(6, Cpu::ABS, Cpu::DEC),

            0xd0 => wr(2, Cpu::REL, Cpu::BNE),
            0xd1 => wr(5, Cpu::IDY, Cpu::CMP),
            0xd5 => wr(4, Cpu::ZPX, Cpu::CMP),
            0xd6 => wr(6, Cpu::ZPX, Cpu::DEC),
            0xd8 => wr(2, Cpu::IMP, Cpu::CLD),
            0xd9 => wr(4, Cpu::ABY, Cpu::CMP),
            0xdd => wr(4, Cpu::ABX, Cpu::CMP),
            0xde => wr(7, Cpu::ABX, Cpu::DEC),

            0xe0 => wr(2, Cpu::IMM, Cpu::CPX),
            0xe1 => wr(6, Cpu::IDX, Cpu::SBC),
            0xe4 => wr(3, Cpu::ZPG, Cpu::CPX),
            0xe5 => wr(3, Cpu::ZPG, Cpu::SBC),
            0xe6 => wr(5, Cpu::ZPG, Cpu::INC),
            0xe8 => wr(2, Cpu::IMP, Cpu::INX),
            0xe9 => wr(2, Cpu::IMM, Cpu::SBC),
            0xea => wr(2, Cpu::IMP, Cpu::NOP),
            0xec => wr(4, Cpu::ABS, Cpu::CPX),
            0xed => wr(4, Cpu::ABS, Cpu::SBC),
            0xee => wr(6, Cpu::ABS, Cpu::INC),

            0xf0 => wr(2, Cpu::REL, Cpu::BEQ),
            0xf1 => wr(5, Cpu::IDY, Cpu::SBC),
            0xf5 => wr(4, Cpu::ZPX, Cpu::SBC),
            0xf6 => wr(6, Cpu::ZPX, Cpu::INC),
            0xf8 => wr(2, Cpu::IMP, Cpu::SED),
            0xf9 => wr(4, Cpu::ABY, Cpu::SBC),
            0xfd => wr(4, Cpu::ABX, Cpu::SBC),
            0xfe => wr(7, Cpu::ABX, Cpu::INC),

            _ => wr(1, Cpu::XXX, Cpu::XXX)
        }
    }

    
}

// wrapper to remove function overhead from decode table
fn wr(cycles: u8, addr_mode: fn(&mut Cpu), instruction: fn(&mut Cpu)) -> DecodedOpcode {
    return DecodedOpcode {
        cycles, addr_mode, instruction
    }
}