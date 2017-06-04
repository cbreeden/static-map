#![feature(proc_macro)]
#[macro_use]
extern crate staticmap_macros;
use staticmap_macros::static_map_macro;

// #[derive(Copy, Clone)]
struct RGB(u8,u8,u8);

struct Map<'a, K: 'a, V: 'a> {
    hashes: &'a [usize],
    entries: &'a [(K, V)],
}

macro_rules! static_map {
    (Default: $default:expr, $($key:expr => $value:expr,)* $(,)*) => ({
        static_map_macro!(
            Default: @$default@
            $($key @$value@ )*
        )
    })
}

static map: Map<&'static str, RGB> = Map {
    hashes: &[0, 0, 0, 0, 0],
    entries: &[ ("red", RGB(255,0,0)), ("blue", RGB(0,0,255)) ],
};

fn main() {
    let mmap: Map<&'static str, RGB> = static_map!(
        Default: RGB(0,0,0),
        "red" => RGB(255,0,0),
        "green" => RGB(0,255,0),
        "blue" => RGB(0,0,255),
    );

    let mm: Map<&'static str, RGB> = Map {
        hashes: &[0, 0, 0, 0, 0],
        entries: &[ ("red", RGB(255,0,0)), ("blue", RGB(0,0,255)) ],
    };

    println!("Hello!");
}