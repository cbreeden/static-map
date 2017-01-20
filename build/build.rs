#![feature(proc_macro)]
extern crate phf_codegen;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate fnv;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

mod fxhash;
mod static_hash;
mod test_data;

use test_data::Glyph;
use test_data::GLYPHS;
use static_hash::Table;

fn main() {
    // make_phfset();
    // make_fnvstatic();
    make_fxstatic();
}

macro_rules! display {
    ($id:expr) => (format!("{}", $id))
}

fn make_phfset() {
    let output = Path::new(&env::var_os("OUT_DIR").expect("OUT_DIR")).join("phf.rs");
    let mut file = BufWriter::new(File::create(&output).expect("glyphs.rs file"));

    let mut map = phf_codegen::Map::new();
    for &(code, glyph) in GLYPHS.iter() {
        map.entry(code, &display!(glyph));
    }

    write!(&mut file, "pub static PHFSET: phf::Set<u32> = ").unwrap();
    map.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}

fn make_fxstatic() {
    let output = Path::new(&env::var_os("OUT_DIR").expect("OUT_DIR")).join("fx.rs");
    let mut file = BufWriter::new(File::create(&output).expect("fx.rs file"));

    let mut t = Table::<u32, Glyph, fxhash::FxHasher>::with_capacity(GLYPHS.len() as u32);
    for &(code, glyph) in GLYPHS.iter() {
        t.insert(code, glyph);
    }

    write!(&mut file, "static FX_MAP: Map<u32, Glyph, fxhash::FxHasher> = Map {{  \
                         entries: &[").unwrap();
    for entry in t.entries.iter() {
        write!(&mut file, "{}, ", entry).unwrap();
    }

    write!(&mut file, "  ],\n").unwrap();

    write!(&mut file, "  _hasher: ::std::marker::PhantomData,").unwrap();
    write!(&mut file, "}};").unwrap();
}