#![feature(test)]
#![allow(dead_code)]

extern crate phf;
extern crate test;
extern crate fnv;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

//extern crate seahash;
//extern crate xxhash2;
//extern crate murmurhash64;

mod fxhash;
mod hashmap;
mod hashbuilder;
mod testdata;
mod hist;

use std::fmt;

use test::Bencher;

use hashmap::{Entry, Map};
use testdata::Glyph;
use testdata::GLYPHS;

// fn bench_hashmap<H>(hasher: H, b: &mut Bencher)
//   where H: Hasher + Default
// {
//   let glyphs: Glyphs =  serde_json::from_reader(&glyph_file).unwrap();
//   let table = make_glyphs::<H>();

//   b.iter(|| {
//     let mut hist = Hist::new();

//     for code in glyphs.0.iter() {
//       if let Some((_, dist)) = table.lookup_index(&code.unicode) {
//         hist.insert(dist as u32);
//       }
//     }
//   })
// }

// #[bench]
// fn fxhasher(b: &mut Bencher) {
//   bench_hashmap::<fxhash::FxHasher>(b);
// }

// #[bench]
// fn fnv(b: &mut Bencher) {
//   bench_hashmap::<fnv::FnvHasher>(b);
// }

// FNV_STATIC_SET
// PNFSET
// include!(concat!(env!("OUT_DIR"), "/glyphs.rs"));
include!(concat!(env!("OUT_DIR"), "/fx.rs"));

#[bench]
fn fx(b: &mut Bencher) {
  b.iter(|| {
    for &(code, glyph) in GLYPHS.iter() {
      assert_eq!(*FX_MAP.get(&code).unwrap(), glyph)
    }
  })
}

// #[bench]
// fn static_fnv(b: &mut Bencher) {
//   let glyph_file = File::open("glyphs.json").unwrap();
//   let json: Glyphs = serde_json::from_reader(&glyph_file).unwrap();

//   b.iter(|| {
//     let mut hist = Hist::new();

//     for glyph in json.0.iter() {
//       if let Some((_, dist)) = FNV_STATIC_SET.lookup_index(&glyph.unicode) {
//         hist.insert(dist as u32)
//       }
//     }
//   })
// }

// #[bench]
// fn phf(b: &mut Bencher) {
//   let glyph_file = File::open("glyphs.json").unwrap();
//   let json: Glyphs = serde_json::from_reader(&glyph_file).unwrap();

//   b.iter(|| {
//     let mut hist = Hist::new();

//     for code in json.0.iter() {
//       if PHFSET.contains(&code.unicode) {
//         hist.insert(1 as u32);
//       }
//     }
//   })
// }

fn main() {
  println!("I'm only used for tests, I should probably be a library.")
}