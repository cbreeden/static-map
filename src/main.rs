#![feature(test)]
#![allow(dead_code)]
extern crate phf;

extern crate test;
use test::Bencher;

extern crate fnv;
extern crate seahash;
extern crate xxhash2;
extern crate murmurhash64;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::cmp;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;

use std::fmt;
use std::fmt::Debug;
use std::fs::File;

const MIN_TABLE_SIZE: usize = 32;

#[derive(Default, Debug)]
struct Entry<K> {
  hash: usize,
  key:  K,
}

struct Table<K, H> {
  mask:     usize,
  entries:  Vec<Entry<K>>,
  _hasher:   PhantomData<H>,
}

impl<K, H> Table<K, H>
    where K: Hash + Default + Eq + Debug,
          H: Hasher + Default {
  fn with_capacity(size: u32) -> Table<K, H> {
    // Table size must be a power of two.
    let cap = cmp::max((size * 10/9).next_power_of_two() as usize, MIN_TABLE_SIZE);
    let mut entries = Vec::with_capacity(cap);

    for _ in 0..cap {
      entries.push(Entry::<K>::default());
    }

    Table {
      mask:    cap - 1,
      entries: entries,
      _hasher:  PhantomData,
    }
  }

  fn insert(&mut self, mut key: K) {
    // Hash the given key, determine ideal index
    let mut hash = Self::hash(&key);
    let mut pos  = hash & self.mask as usize;
    let mut dist = 0;

    loop {
      let current_entry = unsafe { self.entries.get_unchecked_mut(pos) };

      // Found an empty bucket.  Place hash and return.
      if current_entry.hash == 0 {
        current_entry.hash = hash;
        current_entry.key =  key;
        return
      }

      // Check if current key has an ideal dist less than held hash.
      // If so, replace current hash with held hash, update new dist
      // and continue.
      let ideal = current_entry.hash;
      let current_dist = pos.wrapping_sub(ideal) & self.mask;

      if current_dist < dist {
        std::mem::swap(&mut key, &mut current_entry.key);
        std::mem::swap(&mut hash, &mut current_entry.hash);
        dist = current_dist;
      }

      pos = (pos + 1) & self.mask;
      dist += 1;
    }
  }

  fn lookup_index(&self, key: &K) -> Option<(usize, usize)> {
    let hash = Self::hash(key);
    let mut pos  = hash & self.mask;
    let mut dist = 1;

    loop {
      let current_entry = unsafe { self.entries.get_unchecked(pos) };
      if current_entry.hash == hash && self.entries[pos].key == *key {
        return Some((pos, dist))
      } else if current_entry.hash == 0 {
        return None
      }

      pos = (pos + 1) & self.mask;
      dist += 1;
    }
  }

  fn hash(key: &K) -> usize {
    let mut hasher = H::default();
    key.hash(&mut hasher);
    let hash =  hasher.finish() as usize;
    if hash == 0 { 1 } else { hash }
  }
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

//const SIZE: u32 = 10000;

fn make_glyphs<H>() -> Table<u32, H>
    where H: Hasher + Default
{
    // Read glyph data from `build/glyphs.json`
    let glyph_file = File::open("glyphs.json").unwrap();
    let json: Glyphs = serde_json::from_reader(&glyph_file).unwrap();

    let size = json.0.len() as u32 + SHIM.len() as u32;
    let mut t = Table::<u32, H>::with_capacity(size);

    for glyph in json.0.iter().take(size as usize) {
        t.insert(glyph.unicode);
    }

    // Insert shim
    for &(_, old) in SHIM.iter() {
        let idx = json.0.binary_search_by_key(&old, |ref g| g.unicode).unwrap();
        t.insert(json.0[idx].unicode)
    }

    let mut hist = Hist::new();
    for glyph in &json.0 {
      if let Some((_, dist)) = t.lookup_index(&glyph.unicode) {
        hist.insert(dist as u32);
      }
    }

    println!("{}", hist);
    t
}

fn main() {
  macro_rules! make_glyphs {
    ($hasher:ty) => ({
      println!("Hasher: {}", stringify!($hasher));
      make_glyphs::<$hasher>();
    })
  }

  make_glyphs!(fnv::FnvHasher);
  make_glyphs!(seahash::SeaHasher);
  make_glyphs!(xxhash2::State64);
}

macro_rules! make_glyphs {
  ($hasher:ty) => ({
    println!("Hasher: {}", stringify!($hasher));
    make_glyphs::<$hasher>()
  })
}

#[bench]
fn fnv(b: &mut Bencher) {
  let glyph_file = File::open("glyphs.json").unwrap();
  let json: Glyphs = serde_json::from_reader(&glyph_file).unwrap();
  let table = make_glyphs!(fnv::FnvHasher);

  b.iter(|| {
    let mut hist = Hist::new();

    for code in json.0.iter() {
      if let Some((_, dist)) = table.lookup_index(&code.unicode) {
        hist.insert(1 as u32);
      }
    }
  })
}

#[bench]
fn seahash(b: &mut Bencher) {
  let glyph_file = File::open("glyphs.json").unwrap();
  let json: Glyphs = serde_json::from_reader(&glyph_file).unwrap();
  let table = make_glyphs!(seahash::SeaHasher);

  b.iter(|| {
    let mut hist = Hist::new();

    for code in json.0.iter() {
      if let Some((_, dist)) = table.lookup_index(&code.unicode) {
        hist.insert(1 as u32);
      }
    }
  })
}

#[bench]
fn xxhash(b: &mut Bencher) {
  let glyph_file = File::open("glyphs.json").unwrap();
  let json: Glyphs = serde_json::from_reader(&glyph_file).unwrap();
  let table = make_glyphs!(xxhash2::State64);

  b.iter(|| {
    let mut hist = Hist::new();

    for code in json.0.iter() {
      if let Some((_, dist)) = table.lookup_index(&code.unicode) {
        hist.insert(1 as u32);
      }
    }
  })
}

include!(concat!(env!("OUT_DIR"), "/glyphs.rs"));

#[bench]
fn phf(b: &mut Bencher) {
  let glyph_file = File::open("glyphs.json").unwrap();
  let json: Glyphs = serde_json::from_reader(&glyph_file).unwrap();

  b.iter(|| {
    let mut hist = Hist::new();

    for code in json.0.iter() {
      if let Some(_) = phftable.get(&code.unicode) {
        hist.insert(1 as u32);
      }
    }
  })
}

struct Hist(Vec<u32>);

impl Hist {
  fn new() -> Hist {
    Hist(Vec::new())
  }

  fn insert(&mut self, val: u32) {
    if val < self.0.len() as u32 {
      self.0[val as usize] += 1;
    } else {
      // extend with zeros
      let n = val as usize - self.0.len();
      for _ in 0..n { self.0.push(0) }
      self.0.push(1);
    }
  }
}

impl fmt::Display for Hist {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for (idx, val) in self.0.iter().enumerate().skip_while(|&(_, &val)| val == 0) {
      writeln!(f, "{} => {}", idx, val)?;
    }
    Ok(())
  }
}

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