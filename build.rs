#![feature(proc_macro)]
extern crate phf_codegen;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate fnv;

use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::Read;
use std::path::Path;

mod fxhash;
mod static_hash;
use static_hash::Table;

fn main() {
    make_phfset();
    make_fnvstatic();
}

#[derive(Deserialize, Debug)]
struct Glyph {
    unicode: u32,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    advance: u16,
    lsb: i16,
    italics: i16,
    attachment: i16,
}

#[derive(Deserialize, Debug)]
struct Glyphs(pub Vec<Glyph>);

impl fmt::Display for Glyph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "Glyph {{ unicode: {}, \
                        bbox: BBox({}, {}, {}, {}), \
                        advance: {}, \
                        lsb: {}, \
                        italics: {}, \
                        attachment: {} }}",
            self.unicode,
            self.min_x, self.min_y, self.max_x, self.max_y,
            self.advance,
            self.lsb,
            self.italics,
            self.attachment)
    }
}

macro_rules! display {
    ($id:expr) => (format!("{}", $id))
}

// Unicode shim for internal representation,
// to pactch some missing glyphs.

const SHIM: [(u32, u32); 24] = [
    // Calligraphic
    (0x1D49D, 0x212C), // B
    (0x1D4A0, 0x2130), // E
    (0x1D4A1, 0x2131), // F
    (0x1D4A3, 0x210B), // H
    (0x1D4A4, 0x2110), // I
    (0x1D4A7, 0x2112), // L
    (0x1D4A8, 0x2133), // M
    (0x1D4AD, 0x211B), // R
    (0x1D4BA, 0x212F), // e
    (0x1D4BC, 0x210A), // g
    (0x1D4C4, 0x2134), // o

    // Blackboard
    (0x1D53A, 0x2102), // C
    (0x1D53F, 0x210D), // H
    (0x1D545, 0x2115), // N
    (0x1D547, 0x2119), // P
    (0x1D548, 0x211A), // Q
    (0x1D549, 0x211D), // R
    (0x1D551, 0x2124), // Z

    // Fracture
    (0x1D506, 0x212D), // C
    (0x1D50B, 0x210C), // H
    (0x1D50C, 0x2111), // I
    (0x1D515, 0x211C), // R
    (0x1D51D, 0x2128), // Z

    // Italic
    (0x1D455, 0x210E), // h
];

fn make_phfset() {
    // Read glyph data from `build/glyphs.json`
    let mut glyph_file = File::open("glyphs.json")
        .expect("Unable to open build/glyphs.json");

    let mut buffer = String::new();
    glyph_file.read_to_string(&mut buffer)
        .expect("Unable to read build/glyphs.json");

    let json: Glyphs = serde_json::from_str(&buffer).unwrap();

    let output = Path::new(&env::var_os("OUT_DIR").expect("OUT_DIR")).join("glyphs.rs");
    let mut file = BufWriter::new(File::create(&output).expect("glyphs.rs file"));

    write!(&mut file, "pub static PHFSET: phf::Set<u32> = ").unwrap();

    let mut map = phf_codegen::Set::new();
    for glyph in &json.0 {
        let code = glyph.unicode;
        map.entry(code);
    }

    // Insert shim
    for &(new, _) in SHIM.iter() {
        //let idx = json.0.binary_search_by_key(&old, |ref g| g.unicode).unwrap();
        map.entry(new);
    }

    map.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}

fn make_fnvstatic() {
    // Read glyph data from `build/glyphs.json`
    let glyph_file = File::open("glyphs.json").unwrap();
    let json: Glyphs = serde_json::from_reader(&glyph_file).unwrap();

    let size = json.0.len() as u32 + SHIM.len() as u32;
    let mut t = Table::<u32, fxhash::FxHasher>::with_capacity(size);

    for glyph in json.0.iter().take(size as usize) {
        t.insert(glyph.unicode);
    }

    // Insert shim
    for &(_, old) in SHIM.iter() {
        let idx = json.0.binary_search_by_key(&old, |ref g| g.unicode).unwrap();
        t.insert(json.0[idx].unicode)
    }

    // Write Static Hash
    let output = Path::new(&env::var_os("OUT_DIR").expect("OUT_DIR")).join("fnv_static.rs");
    let mut file = BufWriter::new(File::create(&output).expect("glyphs.rs file"));

    write!(&mut file, "static FNV_STATIC_SET: StaticHashSet<u32, fxhash::FxHasher> = StaticHashSet {{  \
                         mask: {},  \
                         entries: &[", t.entries.len() - 1).unwrap();
    for entry in t.entries {
        write!(&mut file, "{}, ", entry);
    }

    write!(&mut file, "  ],").unwrap();
    write!(&mut file, "  _hasher: ::std::marker::PhantomData,").unwrap();
    write!(&mut file, "}};").unwrap();
}