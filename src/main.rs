extern crate fxhash;

fn main() {
    let fx = fxhash::FxHasher::default();
    let mut smb = Builder::with_capacity(2, fx);

    for idx in 1..6 {
        smb.insert(idx, idx);
    }

    let mut res = Vec::new();
    let _ = smb.build(&mut res);
    println!("{}", String::from_utf8(res).unwrap());
}