use std::fmt;

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
