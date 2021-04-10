pub trait Mapper {
    fn read_prg(&self, addr: u16) -> u16;
    fn write_prg(&self, addr: u16) -> u16;
    fn read_chr(&self, addr: u16) -> u16;
    fn write_chr(&self, addr: u16) -> u16;
}