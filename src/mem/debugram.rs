use crate::mem::mem::Mem;

pub struct DebugRam {
    pub ram: Vec<u8>
}


impl DebugRam {
    
    pub fn new(program: Vec<u8>) -> Self {

        let mut ram = program;
        ram.resize(0x10000, 0x00);

        return Self {
            ram
        };
    }
}

impl Mem for DebugRam {

    fn read(&self, addr: u16) -> u8 {
        return self.ram[addr as usize];
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}