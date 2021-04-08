use crate::cpu::cpu::Cpu;

#[allow(non_snake_case)]
impl Cpu {

    pub fn ABS(&mut self) {
        self.addr_abs = self.pc_fetch_word();
        self.data = self.read(self.addr_abs);
    }

    pub fn ABX(&mut self) {
        self.ABS();
        self.addr_abs += self.x as u16;
        self.data = self.read(self.addr_abs);
    }

    pub fn ABY(&mut self) {
        self.ABS();
        self.addr_abs += self.y as u16;
        self.data = self.read(self.addr_abs);
    }

    pub fn IMM(&mut self) {
        self.addr_abs = self.pc;
        self.pc += 1;
        self.data = self.read(self.addr_abs);
    }

    pub fn IMP(&mut self) {
        self.data = self.ac;
    }

    pub fn IND(&mut self) {
        // apparently there is a bug here
        let ptr = self.pc_fetch_word();
        let addr = self.fetch_word(ptr);
        self.addr_abs = addr;
        self.data = self.read(self.addr_abs);
    }

    pub fn IDX(&mut self) {
        let ptr = self.pc_fetch_byte() as u16;
        let lo = self.read((ptr + self.x as u16) & 0x00ff) as u16;
        let hi = self.read((ptr + self.x as u16 + 1) & 0x00ff) as u16;
        self.addr_abs = (hi << 8) | lo;
        self.data = self.read(self.addr_abs);
    }

    pub fn IDY(&mut self) {
        let ptr = self.pc_fetch_byte() as u16;
        let lo = self.read((ptr + self.y as u16) & 0x00ff) as u16;
        let hi = self.read((ptr + self.y as u16 + 1) & 0x00ff) as u16;
        self.addr_abs = (hi << 8) | lo;
        self.data = self.read(self.addr_abs);
    }

    pub fn REL(&mut self) {
        self.addr_rel = self.pc_fetch_byte() as u16;
        if self.addr_rel & 0x0080 != 0 {
            self.addr_rel |= 0xff00;
        }
        self.data = self.read(self.addr_abs);
    }

    pub fn ZPG(&mut self) {
        self.addr_abs = self.pc_fetch_byte() as u16;
        self.addr_abs &= 0x00ff;
        self.data = self.read(self.addr_abs);
    }

    pub fn ZPX(&mut self) {
        self.addr_abs = (self.pc_fetch_byte() + self.x) as u16;
        self.addr_abs &= 0x00ff;
        self.data = self.read(self.addr_abs);
    }

    pub fn ZPY(&mut self) {
        self.addr_abs = (self.pc_fetch_byte() + self.y) as u16;
        self.addr_abs &= 0x00ff;
        self.data = self.read(self.addr_abs);
    }
}