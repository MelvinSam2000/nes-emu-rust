use crate::cpu::cpu::*;

#[allow(non_snake_case)]
impl Cpu {

    pub fn ADC(&mut self) {

        let mut temp: u16 = self.ac.wrapping_add(self.data) as u16;
        temp = temp.wrapping_add(self.get_flag(CpuFlag::C) as u16);
	
        self.set_flag(CpuFlag::C, temp > 255);
        self.set_flag(CpuFlag::Z, (temp & 0x00ff) == 0);
        self.set_flag(CpuFlag::V, (!(self.ac as u16 ^ self.data as u16) & (self.ac as u16 ^ temp)) & 0x0080 != 0);
        self.set_flag(CpuFlag::N, temp & 0x80 != 0);
        
        self.ac = temp as u8;
        self.cycles += 1;
    }

    pub fn AND(&mut self) {
        self.ac &= self.data;
        self.set_flag(CpuFlag::Z, self.ac == 0x00);
        self.set_flag(CpuFlag::N, self.ac & 0x80 != 0);
    }

    pub fn ASL(&mut self) {
        let tmp = (self.data as u16) << 1;
        
        self.set_flag(CpuFlag::C, tmp & 0xff00 > 0);
        self.set_flag(CpuFlag::Z, tmp & 0xff00 == 0);
        self.set_flag(CpuFlag::N, tmp & 0x0080 != 0);

        if self.addr_mode == Cpu::IMP as usize {
            self.ac = tmp as u8;
        } else {
            self.write(self.addr_abs, tmp as u8);
        }
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
        let tmp = self.ac & self.data;
        self.set_flag(CpuFlag::Z, tmp & 0x00ff == 0x00);
        self.set_flag(CpuFlag::N, self.data & (1 << 7) != 0);
        self.set_flag(CpuFlag::V, self.data & (1 << 6) != 0);
    }

    pub fn BMI(&mut self) {
        if self.get_flag(CpuFlag::N) {
            self.cycles += 1;
            let addr = self.pc.wrapping_add(self.addr_rel);
            self.pc = addr;
        }
    }

    pub fn BNE(&mut self) {
        if !self.get_flag(CpuFlag::Z) {
            self.cycles += 1;
            let addr = self.pc.wrapping_add(self.addr_rel);
            self.pc = addr;
        }
    }

    pub fn BPL(&mut self) {
        if !self.get_flag(CpuFlag::N) {
            self.cycles += 1;
            let addr = self.pc.wrapping_add(self.addr_rel);
            self.pc = addr;
        }
    }

    pub fn BRK(&mut self) {
        self.pc = self.pc.wrapping_add(1);
	
        self.set_flag(CpuFlag::I, true);
        self.write(0x0100 + self.sp as u16, (self.pc >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(0x0100 + self.sp as u16, self.pc as u8);
        self.sp = self.sp.wrapping_sub(1);

        self.set_flag(CpuFlag::B, true);
        self.write(0x0100 + self.sp as u16, self.status);
        self.sp = self.sp.wrapping_sub(1);
        self.set_flag(CpuFlag::B, false);

        self.pc = self.read(0xfffe) as u16 | ((self.read(0xffff) as u16) << 8);
    }

    pub fn BVC(&mut self) {
        if !self.get_flag(CpuFlag::V) {
            self.cycles += 1;
            let addr = self.pc.wrapping_add(self.addr_rel);
            self.pc = addr;
        }
    }

    pub fn BVS(&mut self) {
        if self.get_flag(CpuFlag::V) {
            self.cycles += 1;
            let addr = self.pc.wrapping_add(self.addr_rel);
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
        self.set_flag(CpuFlag::I, false);
    }

    pub fn CLV(&mut self) {
        self.set_flag(CpuFlag::V, false);
    }

    pub fn CMP(&mut self) {
        let tmp = (self.ac as u16).wrapping_sub(self.data as u16);
        self.set_flag(CpuFlag::C, self.ac >= self.data);
        self.set_flag(CpuFlag::Z, (tmp & 0x00ff) == 0);
        self.set_flag(CpuFlag::N, tmp & 0x0080 != 0);
        self.cycles += 1;
    }

    pub fn CPX(&mut self) {
        let tmp = (self.x as u16).wrapping_sub(self.data as u16);
        self.set_flag(CpuFlag::C, self.x >= self.data);
        self.set_flag(CpuFlag::Z, (tmp & 0x00ff) == 0);
        self.set_flag(CpuFlag::N, tmp & 0x0080 != 0);
    }

    pub fn CPY(&mut self) {
        let tmp = (self.y as u16).wrapping_sub(self.data as u16);
        self.set_flag(CpuFlag::C, self.y >= self.data);
        self.set_flag(CpuFlag::Z, (tmp & 0x00ff) == 0);
        self.set_flag(CpuFlag::N, tmp & 0x0080 != 0);
    }

    pub fn DEC(&mut self) {
	    let tmp = self.data - 1;
	    self.write(self.addr_abs, tmp);
        self.set_flag(CpuFlag::Z, tmp == 0);
        self.set_flag(CpuFlag::N, tmp & 0x0080 != 0);
    }

    pub fn DEX(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.set_flag(CpuFlag::Z, self.x == 0);
        self.set_flag(CpuFlag::N, self.x & 0x0080 != 0);
    }

    pub fn DEY(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.set_flag(CpuFlag::Z, self.y == 0);
        self.set_flag(CpuFlag::N, self.y & 0x0080 != 0);
    }

    pub fn EOR(&mut self) {
        self.ac ^= self.data;
        self.set_flag(CpuFlag::Z, self.ac == 0);
        self.set_flag(CpuFlag::N, self.ac & 0x0080 != 0);
    }

    pub fn INC(&mut self) {
        let tmp = self.data.wrapping_add(1);
	    self.write(self.addr_abs, tmp);
        self.set_flag(CpuFlag::Z, tmp == 0);
        self.set_flag(CpuFlag::N, tmp & 0x0080 != 0);
    }

    pub fn INX(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.set_flag(CpuFlag::Z, self.x == 0);
        self.set_flag(CpuFlag::N, self.x & 0x0080 != 0);
    }

    pub fn INY(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.set_flag(CpuFlag::Z, self.y == 0);
        self.set_flag(CpuFlag::N, self.y & 0x0080 != 0);
    }

    pub fn JMP(&mut self) {
        self.pc = self.addr_abs;
    }

    pub fn JSR(&mut self) {
        self.pc = self.sp.wrapping_sub(1) as u16;
        self.write(self.sp as u16 + 0x100, ((self.pc >> 8) & 0x00ff) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(self.sp as u16 + 0x100, self.pc as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.pc = self.addr_abs;
    }

    pub fn LDA(&mut self) {
        self.ac = self.data;
        self.set_flag(CpuFlag::Z, self.ac == 0);
        self.set_flag(CpuFlag::N, self.ac & 0x0080 != 0);
    }

    pub fn LDX(&mut self) {
        self.x = self.data;
        self.set_flag(CpuFlag::Z, self.x == 0);
        self.set_flag(CpuFlag::N, self.x & 0x0080 != 0);
    }

    pub fn LDY(&mut self) {
        self.y = self.data;
        self.set_flag(CpuFlag::Z, self.y == 0);
        self.set_flag(CpuFlag::N, self.y & 0x0080 != 0);
    }

    pub fn LSR(&mut self) {

        self.set_flag(CpuFlag::C, self.data & 0x0001 != 0);

        let tmp = (self.data as u16) >> 1;
        
        self.set_flag(CpuFlag::Z, tmp & 0x00ff == 0);
        self.set_flag(CpuFlag::N, tmp & 0x0080 != 0);

        if self.addr_mode == Cpu::IMP as usize {
            self.ac = tmp as u8;
        } else {
            self.write(self.addr_abs, tmp as u8);
        }
    }

    pub fn NOP(&mut self) {
        return;
    }

    pub fn ORA(&mut self) {
        self.ac |= self.data;
        self.set_flag(CpuFlag::Z, self.ac == 0x00);
        self.set_flag(CpuFlag::N, self.ac & 0x80 != 0);
    }

    pub fn PHA(&mut self) {
        self.write(self.sp as u16 + 0x0100, self.ac);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn PHP(&mut self) {
        self.write(0x0100 + self.sp as u16, self.status | CpuFlag::B as u8);
        self.set_flag(CpuFlag::B, false);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn PLA(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        self.ac = self.read(self.sp as u16 + 0x0100);
        self.set_flag(CpuFlag::Z, self.ac == 0x00);
        self.set_flag(CpuFlag::N, self.ac & 0x80 != 0);
    }

    pub fn PLP(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        self.read(0x0100 + self.sp as u16);
        self.set_flag(CpuFlag::B, true);
    }

    pub fn ROL(&mut self) {
        let mut tmp = (self.data as u16) << 1;
        if self.get_flag(CpuFlag::C) {
            tmp |= 0x0001;
        }
        
        self.set_flag(CpuFlag::C, tmp & 0xff00 != 0);
        self.set_flag(CpuFlag::Z, tmp & 0x00ff == 0);
        self.set_flag(CpuFlag::N, tmp & 0x0080 != 0);

        if self.addr_mode == Cpu::IMP as usize {
            self.ac = tmp as u8;
        } else {
            self.write(self.addr_abs, tmp as u8);
        }
    }

    pub fn ROR(&mut self) {
        let mut tmp = (self.data as u16) >> 1;
        if self.get_flag(CpuFlag::C) {
            tmp |= 0x0080;
        }
        
        self.set_flag(CpuFlag::C, self.data & 0x0001 != 0);
        self.set_flag(CpuFlag::Z, tmp & 0x00ff == 0);
        self.set_flag(CpuFlag::N, tmp & 0x0080 != 0);

        if self.addr_mode == Cpu::IMP as usize {
            self.ac = tmp as u8;
        } else {
            self.write(self.addr_abs, tmp as u8);
        }
    }

    pub fn RTI(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        self.status = self.read(self.sp as u16 + 0x0100);
        self.set_flag(CpuFlag::B, false);
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.read(self.sp as u16 + 0x0100) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc |= (self.read(self.sp as u16 + 0x0100) as u16) << 8;
    }

    pub fn RTS(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.read(0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc |= (self.read(0x0100 + self.sp as u16) as u16) << 8;
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn SBC(&mut self) {
        let temp = (self.ac + self.data + self.get_flag(CpuFlag::C) as u8) as u16;
        self.data ^= 0xff;
	
        self.set_flag(CpuFlag::C, temp > 255);
        self.set_flag(CpuFlag::Z, (temp & 0x00ff) == 0);
        self.set_flag(CpuFlag::V, (!(self.ac as u16 ^ self.data as u16) & (self.ac as u16 ^ temp)) & 0x0080 != 0);
        self.set_flag(CpuFlag::N, temp & 0x80 != 0);
        
        self.ac = temp as u8;
        self.cycles += 1;
    }

    pub fn SEC(&mut self) {
        self.set_flag(CpuFlag::C, true);
    }

    pub fn SED(&mut self) {
        self.set_flag(CpuFlag::D, true);
    }

    pub fn SEI(&mut self) {
        self.set_flag(CpuFlag::I, true);
    }

    pub fn STA(&mut self) {
        self.write(self.addr_abs, self.ac);
    }

    pub fn STX(&mut self) {
        self.write(self.addr_abs, self.x);
    }

    pub fn STY(&mut self) {
        self.write(self.addr_abs, self.y);
    }

    pub fn TAX(&mut self) {
        self.x = self.ac;
        self.set_flag(CpuFlag::Z, self.x == 0);
        self.set_flag(CpuFlag::N, self.x & 0x0080 != 0);
    }

    pub fn TAY(&mut self) {
        self.y = self.ac;
        self.set_flag(CpuFlag::Z, self.y == 0);
        self.set_flag(CpuFlag::N, self.y & 0x0080 != 0);
    }

    pub fn TSX(&mut self) {
        self.x = self.sp;
        self.set_flag(CpuFlag::Z, self.x == 0);
        self.set_flag(CpuFlag::N, self.x & 0x0080 != 0);
    }

    pub fn TXA(&mut self) {
        self.ac = self.x;
        self.set_flag(CpuFlag::Z, self.ac == 0);
        self.set_flag(CpuFlag::N, self.ac & 0x0080 != 0);
    }

    pub fn TXS(&mut self) {
        self.sp = self.x;
    }

    pub fn TYA(&mut self) {
        self.ac = self.y;
        self.set_flag(CpuFlag::Z, self.ac == 0);
        self.set_flag(CpuFlag::N, self.ac & 0x0080 != 0);
    }

    pub fn XXX(&mut self) {
        return;
    }
    
}