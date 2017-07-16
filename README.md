# Static Map

A static round-robin hashmap implementation, using the same hasher found in rustc. This is currently written using
a bang variant of procedural macros that is only avialable in nightly at the moment.  Procedural macros are currently
in active development, so I woulnd't expect any kind of stability here for the moment.  This was just an exercise
to gain experience for how procedural macro development may become.

## Example

```rust
#[macro_use]
extern crate static_map;
#[macro_use]
extern crate static_map_macros;

use static_map::Map;

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
We need a default value for those slots and you declare it as such.  PHF maps do not require this since they are, well, perfect.
Also note that `#[macro_use]` is required for _both_ `static_map` and `static_map_macros`.  This is because `proc_macros` are not
allowed to export items, so the `static_map!` macro lives in the `static_map` crate. (Note: This is not entirely true, and may be fixed soon).

## Benchmarks

The idea is that a static round-robin hashmap maybe more efficient than PHF for certain datasets.  The trade-off is that a PHF hashmap
implementation is nearly optimal with respect to memory efficiency; `static_map!` may use upto 2x as much memory in the worst case scenario.
Keep that in mind, here are some benchmarks (found in `static_map_macro/benches`).

### CSS Colors

This contains about 150 `&str -> RGB(u8, u8, u8)` entries (like in the example above).

```
test bench_phf       ... bench:       2,027 ns/iter (+/- 224)
test bench_static_map ... bench:         935 ns/iter (+/- 90)
```

### Codepoints

This benchmark contains about 4500 `u32 -> GlyphMetrics` entries.

```
test bench_phf       ... bench:      44,502 ns/iter (+/- 3,971)
test bench_static_map ... bench:      13,097 ns/iter (+/- 2,768)
```

### TeX Symbols

This benchmark contains about 1500 `&str -> Symbol` entries, mapping tex symbols to codepoints.

```
test bench_phf        ... bench:      20,382 ns/iter (+/- 133)
test bench_static_map ... bench:      24,589 ns/iter (+/- 188)
```
