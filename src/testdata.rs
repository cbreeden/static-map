use serde_json;
use std::fs::File;

#[derive(Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub struct Glyph {
    pub unicode: u32,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub advance: u16,
    pub lsb: i16,
    pub italics: i16,
    pub attachment: i16,
}

#[derive(Deserialize)]
pub struct Glyphs(pub Vec<Glyph>);

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

lazy_static! {
  pub static ref GLYPHS: Vec<(u32, Glyph)> = {
    // Read glyph data from `build/glyphs.json`
    let glyph_file = File::open("build/glyphs.json").unwrap();
    let mut glyphs = serde_json::from_reader::<_, Glyphs>(&glyph_file)
      .unwrap().0
      .into_iter()
      .map(|g| (g.unicode, g))
      .collect::<Vec<(u32, Glyph)>>();

    for &(new, old) in SHIM.iter() {
      let idx = glyphs
        .binary_search_by_key(&old, |&(c, _)| c)
        .unwrap();
      let glyph = glyphs[idx].1;
      glyphs.push((new, glyph));
    }

    glyphs
  };
}

use serde_json::Value;
use serde_json::Map;
lazy_static! {
  pub static ref SYMBOLS: Map<String, Value> = {
    let glyph_file = File::open("build/symbols.json").unwrap();
    match serde_json::from_reader(&glyph_file).unwrap() {
      Value::Object(obj) => obj,
      _ => panic!("No object!")
    }
  };
}