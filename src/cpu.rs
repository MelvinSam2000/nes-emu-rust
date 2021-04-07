use crate::buscpu::*;

pub struct Cpu {
    // devices
    bus: BusCpu,
    
    pc: u16,
    ac: u8,
    x: u8,
    y: u8,
    sp: u8,
    status: u8,

}

impl Cpu {
    pub fn new(bus: BusCpu) -> Self {
        return Self {
            bus,
            pc: 0x0000,
            ac: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0x00,
            status: 0x00
        }
    }

    pub fn reset(&mut self) {
        println!("CPU RESET");
    }

    pub fn clock(&mut self) {
        println!("CPU CLOCK");
    }

    pub fn irq(&mut self) {
        println!("CPU IRQ");
    }

    pub fn nmi(&mut self) {
        println!("CPU NMI");
    }

    fn read(&mut self, addr: u16) {
        self.bus.read(addr, false);
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data);
    }
}