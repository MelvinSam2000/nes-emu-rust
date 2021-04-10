pub struct CartChr {
    mem: Vec<u8>
}

impl CartChr {

    pub fn new() -> Self {
        return Self {
            mem: vec![]
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        self.mem.resize(new_size, 0);
    }

    pub fn read(&self, addr: u16) -> u8 {
        match self.mem.get(addr as usize) {
            Some(data) => *data,
            None => 0x00
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }
}