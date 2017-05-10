extern crate staticmap_builder;
extern crate fxhash;

use std::io::Cursor;

use staticmap_builder::Builder;

fn main() {
    let fx = fxhash::FxHasher::default();
    let mut smb = Builder::with_capacity(10, fx);

    for idx in 0..10 {
        smb.insert(idx, idx);
    }

    let mut res = Vec::new();
    smb.build(&mut res);
    println!("{}", String::from_utf8(res).unwrap());
}