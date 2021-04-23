pub struct RegLoopy {
    pub reg: u16
}

// coarse_x: 5 (e)
// coarse_y: 5 (d)
// nametable_x: 1 (c)
// nametable_y: 1 (b)
// fine_y: 3 (a)

// format: 0aaa bcdd ddde eeee

impl RegLoopy {

    pub fn new() -> Self {
        return Self {
            reg: 0x00,
        };
    }

    pub fn get_coarse_x(&self) -> u16 {  
        return self.reg & 0b11111;
    } 
    
    pub fn set_coarse_x(&mut self, val: u8) {
        self.reg = (self.reg & !0b11111) | (val as u16 & 0b11111);
    }
    
    pub fn get_coarse_y(&self) -> u16 {  
        return (self.reg & (0b11111 << 5)) >> 5;
    } 
    
    pub fn set_coarse_y(&mut self, val: u8) {  
        self.reg = (self.reg & !(0b11111 << 5)) | (val as u16 & 0b11111) << 5;
    }
    
    pub fn get_nametable_x(&self) -> bool {  
        return ((self.reg & 0x0400) >> 10) != 0; 
    } 
    
    pub fn set_nametable_x(&mut self, val: bool) {  
        if val {
            self.reg |= 0x0400;
        } else {
            self.reg &= !0x0400;
        }
    }
    
    pub fn get_nametable_y(&self) -> bool {  
        return ((self.reg & 0x0800) >> 11) != 0; 
    } 
    
    pub fn set_nametable_y(&mut self, val: bool) {  
        if val {
            self.reg |= 0x0800;
        } else {
            self.reg &= !0x0800;
        }
    }
    
    pub fn get_fine_y(&self) -> u16 {  
        return (self.reg & 0xf000) >> 12;
    } 
    
    pub fn set_fine_y(&mut self, val: u8) {  
        self.reg = (self.reg & !(0b111 << 12)) | (val as u16 & 0b111) << 12;
    }
    
}