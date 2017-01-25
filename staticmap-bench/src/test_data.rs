use serde_json;
use std::fs::File;
use std::fmt;

use serde_json::Value;
use serde_json::Map;
lazy_static! {
  pub static ref SYMBOLS: Map<String, Value> = {
    let glyph_file = File::open("symbols.json").unwrap();
    match serde_json::from_reader(&glyph_file).unwrap() {
      Value::Object(obj) => obj,
      _ => panic!("No object!")
    }
  };
}