pub struct Ram {
    mem: [u8; 0x2000]
}

impl Ram {

    pub fn new() -> Self {
        return Self {
            mem: [0; 0x2000]
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let addr = addr & 0x07ff;
        return self.mem[addr as usize];
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        let addr = addr & 0x07ff;
        self.mem[addr as usize] = data;
    }
}