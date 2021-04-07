pub struct CartPrg {
    mem: Vec<u8>
}

impl CartPrg {

    pub fn new() -> Self {
        return Self {
            mem: vec![]
        }
    }
}