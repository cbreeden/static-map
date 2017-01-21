#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate staticmap_builder;
extern crate staticmap_hashers;
extern crate rand;
extern crate rayon;

mod hist;

use staticmap_builder::Builder;
use staticmap_hashers::fxhash;

use std::fs::File;

use hist::Hist;



#[derive(Deserialize, Debug)]
struct ListInt(Vec<u32>);

// provide staticmap-bench with a list of keys.
fn main() {
    let file = File::open("out.json").unwrap();
    let glyphs = serde_json::from_reader::<_, ListInt>(&file).unwrap();

    // let mut glyphs = Vec::with_capacity(29472);
    // for _ in 0..921 {
    //     glyphs.extend_from_slice(&rand::random::<[u32; 32]>());
    // }

    let mut score = (0, 10.0);
    for _ in 0..100000 {
        let seed = new_key();
        //print!("Key: {:<20}, ", seed);
        let result = test_hash(&glyphs.0[..], seed, false);
        if result < score.1 {
            score = (seed, result)
        }
    }

    println!("");
    println!("Best: {:?}", score);
    test_hash(&glyphs.0[..], score.0, true);
}

fn test_hash(keys: &[u32], seed: usize, print: bool) -> f32 {
    let mut h = Hist::new();
    let builder = fxhash::FxHashBuilder::with_key(seed);
    let mut t = Builder::<u32, u32, _>::with_capacity(keys.len() as u32, builder);
    for &code in keys.iter() {
        let probe = t.insert(code, code);
        h.insert(probe as u32);
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
