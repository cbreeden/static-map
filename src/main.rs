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

mod fxhash;
mod hashmap;
mod testdata;
mod hist;

#[cfg(test)]
use test::Bencher;

use hashmap::Map;
use testdata::Glyph;
#[cfg(test)]
use testdata::GLYPHS;

// PFH_MAP
include!(concat!(env!("OUT_DIR"), "/phf.rs"));

// FX_MAP
// FX_GLYPHS
include!(concat!(env!("OUT_DIR"), "/fx.rs"));

// FX_INLINE_MAP
include!(concat!(env!("OUT_DIR"), "/fx_inline.rs"));

// FNV_MAP
// FNV_GLYPHS
include!(concat!(env!("OUT_DIR"), "/fnv.rs"));

#[bench]
fn fx_inline(b: &mut Bencher) {
  b.iter(|| {
    for &(code, glyph) in GLYPHS.iter() {
      if let Some(&g) = FX_INLINE_MAP.get(&code) {
        if g.unicode != glyph.unicode {
          panic!("");
        }
      }
    }
  })
}

#[bench]
fn fx(b: &mut Bencher) {
  b.iter(|| {
    for &(code, glyph) in GLYPHS.iter() {
      if let Some(&idx) = FX_MAP.get(&code) {
        let g = FX_GLYPHS[idx];
        if g.unicode != glyph.unicode {
          panic!("");
        }
      }
    }
  })
}

#[bench]
fn fnv(b: &mut Bencher) {
  b.iter(|| {

    for &(code, glyph) in GLYPHS.iter() {
      if let Some(&idx) = FNV_MAP.get(&code) {
        let g = FNV_GLYPHS[idx];
        if g.unicode != glyph.unicode {
          panic!("");
        }
      }
    }
  })
}

#[bench]
fn phf(b: &mut Bencher) {
  b.iter(|| {
    for &(code, glyph) in GLYPHS.iter() {
      if PHF_MAP.get(&code).unwrap().unicode != glyph.unicode {
        panic!("")
      }
    }
  })
}

fn main() {
  println!("I'm only used for tests, I should probably be a library.")
}