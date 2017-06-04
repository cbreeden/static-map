# Static Map

A static round-robin hashmap implementation, using the same hasher found in rustc. This is currently written using
a bang variant of procedural macros that is only avialable at nightly at the moment.  Procedural macros are currently
in active development, so I woulnd't expect any kind of stability here for the moment.  This was just an exercise
to gain experience for how procedural macro development may be.

## Example

```rust
#![feature(proc_macro)]

#[macro_use]
extern crate staticmap;
extern crate staticmap_macro;

use staticmap::Map;

#[derive(Clone, Copy)]
struct RGB(u8, u8, u8);

static CSS_COLORS: Map<&'static str, RGB> = static_map! {
    Default: RGB(0x00,0x00,0x00),
    "black" => RGB(0x00,0x00,0x00),
    "silver" => RGB(0xc0,0xc0,0xc0),
    "gray" => RGB(0x80,0x80,0x80),
    "white" => RGB(0xff,0xff,0xff),
    "maroon" => RGB(0x80,0x00,0x00),
    "red" => RGB(0xff,0x00,0x00),
    "purple" => RGB(0x80,0x00,0x80),
    "fuchsia" => RGB(0xff,0x00,0xff),
    "green" => RGB(0x00,0x80,0x00),
};

pub fn rgb_from_str(color: &str) -> Option<RGB> {
    CSS_COLORS.get(color).cloned()
}
```
Notice that `Default: RGB(0, 0, 0),`.  This is required since we are initializing an array which will contain empty slots.
We need a default value for those slots and you declare it as such.

## Benchmarks

The idea is that a static round-robin hashmap maybe more efficient than PHF for certain datasets.  The trade-off is that a PHF hashmap
implementation is nearly optimal with respect to memory efficiency; `static_map!` may use upto 2x as much memory in the worst case scenario.
Keep that in mind, here are some benchmarks (found in `staticmap_macro/benches`).

### CSS Colors

This contains about 150 `&str -> RGB(u8, u8, u8)` entries (like in the example above).  

```
test bench_phf       ... bench:       1,869 ns/iter (+/- 106)
test bench_staticmap ... bench:       1,142 ns/iter (+/- 84)
```

### Codepoints

This benchmark contains about 4500 `u8 -> GlyphMetrics` entries.

```
test bench_phf       ... bench:      43,097 ns/iter (+/- 769)
test bench_staticmap ... bench:      12,688 ns/iter (+/- 353)
```

### TeX Symbols

This benchmark contains about 2500 `&str -> u32` entries, mapping tex symbols to codepoints.

```
test bench_phf       ... bench:      39,305 ns/iter (+/- 629)
test bench_staticmap ... bench:      50,887 ns/iter (+/- 894)
```