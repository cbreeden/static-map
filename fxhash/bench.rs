#![feature(test)]
extern crate test;
use test::{Bencher, black_box};

extern crate fxhash;

macro_rules! generate_benches {
    ($($name:ident, $s:expr),* $(,)*) => (
        $(
            #[bench]
            fn $name(b: &mut Bencher) {
                let s = black_box($s);
                b.iter(|| {
                    fxhash::hash(&s)
                })
            }
        )*
    )
}

generate_benches!(
    bench_3_chars, "123",
    bench_4_chars, "1234",
    bench_11_chars, "12345678901",
    bench_12_chars, "123456789012",
    bench_23_chars, "12345678901234567890123",
    bench_24_chars, "123456789012345678901234",
    bench_67_chars, "1234567890123456789012345678901234567890123456789012345678901234567",
    bench_131_chars, "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901",
);