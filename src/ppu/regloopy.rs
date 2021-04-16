pub struct RegLoopy {
    pub reg: u16,
}

impl RegLoopy {

    pub fn new() -> Self {
        return Self {
            reg: 0x00,
        };
    }
}