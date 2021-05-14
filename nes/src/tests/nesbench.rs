/*
#![feature(test)]
extern crate test;

#[cfg(test)]
mod nesbench {
    use crate::nes::Nes;
    use test::Bencher;

    #[bench]
    fn bench_clock(b: &mut Bencher) {
        let n = Nes::new();
        n.load(String::from("games/nestest.nes"));
        n.reset();
        b.iter(|| n.clock());
    }

    #[bench]
    fn bench_clock_dbg(b: &mut Bencher) {
        let n = Nes::new();
        n.load(String::from("games/nestest.nes"));
        n.reset();
        b.iter(|| n.clock_debug());
    }
}
*/