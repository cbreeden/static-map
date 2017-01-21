#![feature(test)]
#![allow(dead_code)]

extern crate phf;
extern crate test;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate staticmap_hashers;
extern crate staticmap;

#[macro_use]
extern crate lazy_static;

use staticmap_hashers::fxhash;
use staticmap_hashers::fxhash::FxHashBuilder;
use staticmap::Map;

mod testdata;
mod hist;

#[cfg(test)]
use test::Bencher;

use testdata::Glyph;
#[cfg(test)]
use testdata::GLYPHS;

// PFH_MAP
include!(concat!(env!("OUT_DIR"), "/phf.rs"));

// FX_MAP
// FX_GLYPHS
include!(concat!(env!("OUT_DIR"), "/fx.rs"));


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