#![feature(proc_macro)]
extern crate phf_codegen;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate fnv;
#[macro_use]
extern crate lazy_static;
extern crate staticmap_builder;
extern crate staticmap_hashers;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

mod test_data;

use test_data::Glyph;
use test_data::GLYPHS;
use staticmap_builder::Builder;
use staticmap_hashers::fxhash;

fn main() {
    make_phfset();
    // make_fnvstatic();
    make_fxstatic();
    // make_fxinline();
}

macro_rules! display {
    ($id:expr) => (format!("{}", $id))
}

fn make_phfset() {
    let output = Path::new(&env::var_os("OUT_DIR").expect("OUT_DIR")).join("phf.rs");
    let mut file = BufWriter::new(File::create(&output).expect("phf.rs file"));

    let mut map = phf_codegen::Map::new();
    for &(code, glyph) in GLYPHS.iter() {
        map.entry(code, &display!(glyph));
    }

    write!(&mut file, "pub static PHF_MAP: phf::Map<u32, Glyph> = ").unwrap();
    map.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}

fn make_fxstatic() {
    let output = Path::new(&env::var_os("OUT_DIR").expect("OUT_DIR")).join("fx.rs");
    let mut file = BufWriter::new(File::create(&output).expect("fx.rs file"));

    let mut g: Vec<Glyph> = Vec::new();
    let mut t = Builder::<u32, usize, _>::with_capacity(GLYPHS.len() as u32, fxhash::FxHashBuilder::default());

    for &(code, glyph) in GLYPHS.iter() {
        t.insert(code, g.len());
        g.push(glyph);
    }

    write!(&mut file, "static FX_MAP: Map<u32, usize, fxhash::FxHashBuilder> = ").unwrap();
    t.build(&mut file).unwrap();

    write!(&mut file, "static FX_GLYPHS: [Glyph; {}] = [", g.len()).unwrap();

    for glyph in g {
        write!(&mut file, "{}, ", glyph).unwrap();
    }

    write!(&mut file, "];").unwrap();
}

// fn make_fnvstatic() {
//     let output = Path::new(&env::var_os("OUT_DIR").expect("OUT_DIR")).join("fnv.rs");
//     let mut file = BufWriter::new(File::create(&output).expect("fnv.rs file"));

//     let mut g: Vec<Glyph> = Vec::new();
//     let mut t = Builder::<u32, usize, _>::with_capacity(GLYPHS.len() as u32,);

//     for &(code, glyph) in GLYPHS.iter() {
//         t.insert(code, g.len());
//         g.push(glyph);
//     }

//     write!(&mut file, "static FNV_MAP: Map<u32, usize, fxhash::FxHasher> = ").unwrap();
//     t.build(&mut file).unwrap();

//     write!(&mut file, "static FNV_GLYPHS: [Glyph; {}] = [", g.len()).unwrap();

//     for glyph in g {
//         write!(&mut file, "{}, ", glyph).unwrap();
//     }

//     write!(&mut file, "];").unwrap();
// }

// fn make_fxinline() {
//     let output = Path::new(&env::var_os("OUT_DIR").expect("OUT_DIR")).join("fx_inline.rs");
//     let mut file = BufWriter::new(File::create(&output).expect("fx_inline.rs file"));

//     let mut t = Builder::<u32, Glyph, _>::with_capacity(GLYPHS.len() as u32, fxhash::FxHashBuilder::default());

//     for &(code, glyph) in GLYPHS.iter() {
//         t.insert(code, glyph);
//     }

//     write!(&mut file, "static FX_INLINE_MAP: Map<u32, Glyph, _> = ").unwrap();
//     t.build(&mut file).unwrap();
// }