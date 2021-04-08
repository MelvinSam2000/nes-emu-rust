use crate::cpu::cpu::*;

impl Cpu {

    pub fn ADC(&mut self) {

	    let temp = (self.ac + self.data + self.get_flag(CpuFlag::C) as u8) as u16;
	
        self.set_flag(CpuFlag::C, temp > 255);
        self.set_flag(CpuFlag::Z, (temp & 0x00ff) == 0);
        self.set_flag(CpuFlag::V, (!(self.ac as u16 ^ self.data as u16) & (self.ac as u16 ^ temp)) & 0x0080 != 0);
        self.set_flag(CpuFlag::N, temp & 0x80 != 0);
        
        self.ac = temp as u8;
    }

    pub fn AND(&mut self) {
        self.ac &= self.data;
        self.set_flag(CpuFlag::Z, self.ac == 0x00);
        self.set_flag(CpuFlag::N, self.ac & 0x80 != 0);
    }

    pub fn ASL(&mut self) {

    }

    pub fn BCC(&mut self) {
        if !self.get_flag(CpuFlag::C) {
            self.cycles += 1;
            let addr = self.pc + self.addr_rel;
            self.pc = addr;
        }
    }

    pub fn BCS(&mut self) {
        if self.get_flag(CpuFlag::C) {
            self.cycles += 1;
            let addr = self.pc + self.addr_rel;
            self.pc = addr;
        }
    }

    pub fn BEQ(&mut self) {
        if self.get_flag(CpuFlag::Z) {
            self.cycles += 1;
            let addr = self.pc + self.addr_rel;
            self.pc = addr;
        }
    }

    pub fn BIT(&mut self) {

    }

    pub fn BMI(&mut self) {
        if self.get_flag(CpuFlag::N) {
            self.cycles += 1;
            let addr = self.pc + self.addr_rel;
            self.pc = addr;
        }
    }

    pub fn BNE(&mut self) {
        if !self.get_flag(CpuFlag::Z) {
            self.cycles += 1;
            let addr = self.pc + self.addr_rel;
            self.pc = addr;
        }
    }

    pub fn BPL(&mut self) {
        if !self.get_flag(CpuFlag::N) {
            self.cycles += 1;
            let addr = self.pc + self.addr_rel;
            self.pc = addr;
        }
    }

    pub fn BRK(&mut self) {

    }

    pub fn BVC(&mut self) {
        if !self.get_flag(CpuFlag::V) {
            self.cycles += 1;
            let addr = self.pc + self.addr_rel;
            self.pc = addr;
        }
    }

    pub fn BVS(&mut self) {
        if self.get_flag(CpuFlag::V) {
            self.cycles += 1;
            let addr = self.pc + self.addr_rel;
            self.pc = addr;
        }
    }

    pub fn CLC(&mut self) {
        self.set_flag(CpuFlag::C, false);
    }

    pub fn CLD(&mut self) {
        self.set_flag(CpuFlag::D, false);
    }

    pub fn CLI(&mut self) {
        
    }

    pub fn CLV(&mut self) {
        
    }

    pub fn CMP(&mut self) {

    }

    pub fn CPX(&mut self) {

    }

    pub fn CPY(&mut self) {

    }

    pub fn DEC(&mut self) {

    }

    pub fn DEX(&mut self) {

    }

    pub fn DEY(&mut self) {

    }

    pub fn EOR(&mut self) {

    }

    pub fn INC(&mut self) {

    }

    pub fn INX(&mut self) {

    }

    pub fn INY(&mut self) {

    }

    pub fn JMP(&mut self) {

    }

    pub fn JSR(&mut self) {

    }

    pub fn LDA(&mut self) {

    }

    pub fn LDX(&mut self) {

    }

    pub fn LDY(&mut self) {

    }

    pub fn LSR(&mut self) {

    }

    pub fn NOP(&mut self) {

    }

    pub fn ORA(&mut self) {

    }

    pub fn PHA(&mut self) {
        self.write(self.sp as u16 + 0x0100, self.ac);
        self.sp -= 1;
    }

    pub fn PHP(&mut self) {

    }

    pub fn PLA(&mut self) {
        self.sp += 1;
        self.ac = self.read(self.sp as u16 + 0x0100);
        self.set_flag(CpuFlag::Z, self.ac == 0x00);
        self.set_flag(CpuFlag::N, self.ac & 0x80 != 0);
    }

    pub fn PLP(&mut self) {

    }

    pub fn ROL(&mut self) {

    }

    pub fn ROR(&mut self) {

    }

    pub fn RTI(&mut self) {
        self.sp += 1;
        self.status = self.read(self.sp as u16 + 0x0100);
        self.set_flag(CpuFlag::B, false);
        self.sp += 1;
        self.pc = self.read(self.sp as u16 + 0x0100) as u16;
        self.sp += 1;
        self.pc |= (self.read(self.sp as u16 + 0x0100) as u16) << 8;
    }

    pub fn RTS(&mut self) {

    }

    pub fn SBC(&mut self) {
        let temp = (self.ac + self.data + self.get_flag(CpuFlag::C) as u8) as u16;
        self.data ^= 0xff;
	
        self.set_flag(CpuFlag::C, temp > 255);
        self.set_flag(CpuFlag::Z, (temp & 0x00ff) == 0);
        self.set_flag(CpuFlag::V, (!(self.ac as u16 ^ self.data as u16) & (self.ac as u16 ^ temp)) & 0x0080 != 0);
        self.set_flag(CpuFlag::N, temp & 0x80 != 0);
        
        self.ac = temp as u8;
    }

    pub fn SEC(&mut self) {

    }

    pub fn SED(&mut self) {

    }

    pub fn SEI(&mut self) {

    }

    pub fn STA(&mut self) {

    }

    pub fn STX(&mut self) {

    }

    pub fn STY(&mut self) {

    }

    pub fn TAX(&mut self) {

    }

    pub fn TAY(&mut self) {

    }

    pub fn TSX(&mut self) {

    }

    pub fn TXA(&mut self) {

    }

    pub fn TXS(&mut self) {

    }

    pub fn TYA(&mut self) {

    }

    pub fn XXX(&mut self) {

    }
    
}