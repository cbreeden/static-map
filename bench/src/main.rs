#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate staticmap_builder;
extern crate staticmap_hashers;
extern crate rand;
extern crate rayon;
#[macro_use]
extern crate lazy_static;

mod hist;
mod test_data;

use test_data::SYMBOLS;

use staticmap_builder::Builder;
use staticmap_hashers::fxhash;

use std::fs::File;
use hist::Hist;

#[derive(Deserialize, Debug)]
struct ListInt(Vec<u32>);

// provide staticmap-bench with a list of keys.
fn main() {
    // let file = File::open("out.json").unwrap();
    // let glyphs = serde_json::from_reader::<_, ListInt>(&file).unwrap();

    // let mut glyphs = Vec::with_capacity(29472);
    // for _ in 0..921 {
    //     glyphs.extend_from_slice(&rand::random::<[u32; 32]>());
    // }

    let keys = SYMBOLS
        .keys()
        .map(|s| &s[..])
        .collect::<Vec<_>>();

    let mut score = (0, 10.0);
    for _ in 0..1000 {
        let seed = new_key();
        //print!("Key: {:<20}, ", seed);
        let result = test_hash(&keys, seed, false);
        if result < score.1 {
            score = (seed, result)
        }
    }

    println!("");
    println!("Best: {:?}", score);
    test_hash(&keys, score.0, true);
}

macro_rules! display {
    ($expr:expr) => (format!("{}", $expr))
}

fn test_hash(keys: &[&str], seed: usize, print: bool) -> f32 {
    let mut h = Hist::new();
    let builder = fxhash::FxHashBuilder::with_key(seed);
    let mut t = Builder::with_capacity(keys.len(), builder);
    for &code in keys.iter() {
        let probe = t.insert(code, code.len());
        h.insert(probe);
    }

    let avg = h.average();
    if print {
        println!("Avg: {}, {}", avg, h);
    }

    avg
}

fn new_key() -> usize {
    let rng = rand::random::<usize>();
    rng | 1
}
