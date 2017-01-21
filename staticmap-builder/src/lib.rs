use std::cmp;
use std::mem;
use std::marker::PhantomData;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;
use std::hash::BuildHasher;
use std::io;

const MIN_TABLE_SIZE: usize = 32;

#[derive(Default, Debug)]
pub struct Entry<K, V> {
  pub key:   K,
  pub value: V,
}

pub struct Builder<K, V, S> {
  pub hashes:  Vec<usize>,
  pub entries: Vec<Entry<K, V>>,
  pub hasher:  S,
}

impl<K, V, S> Builder<K, V, S>
    where K: Hash + Default + Eq + Display,
          V: Default + Display,
          S: BuildHasher + Debug {
  pub fn with_capacity(size: u32, hasher: S) -> Builder<K, V, S> {
    // Builder size must be a power of two.
    let cap = cmp::max((size * 10/9).next_power_of_two() as usize, MIN_TABLE_SIZE);
    let mut entries = Vec::with_capacity(cap);

    for _ in 0..cap {
      entries.push(Entry::<K, V>::default());
    }

    Builder {
      hashes:  vec![0; cap],
      entries: entries,
      hasher:  hasher,
    }
  }

  pub fn insert(&mut self, key: K, value: V) -> usize {
    let mut hash = self.hash(&key);
    let mask = self.entries.len() - 1;

    let mut entry = Entry {
      key:   key,
      value: value,
    };

    // Hash the given key, determine ideal index
    let mut pos  = hash & mask;
    let mut dist = 0;

    loop {
      let probe_hash = unsafe { self.hashes.get_unchecked_mut(pos) };

      // Found an empty bucket.  Place hash and return.
      if *probe_hash == 0 {
        *probe_hash = hash;
        unsafe {
          let probe = self.entries.get_unchecked_mut(pos);
          *probe = entry;
        }
        return dist
      }

      // Check if current key has an ideal dist less than held hash.
      // If so, replace current hash with held hash, update new dist
      // and continue.
      let probe_dist = pos.wrapping_sub(*probe_hash) & mask;

      if probe_dist < dist {
        unsafe {
          let probe = self.entries.get_unchecked_mut(pos);
          mem::swap(probe, &mut entry);
          mem::swap(probe_hash, &mut hash);
        }
        dist = probe_dist;
      }

      pos = (pos + 1) & mask;
      dist += 1;
    }
  }

  pub fn build<W>(&self, f: &mut W) -> io::Result<()>
    where W: io::Write
  {
    write!(f, "Map {{\n hashes: &[")?;

    for hash in self.hashes.iter() {
        write!(f, "{}, ", hash)?;
    }

    write!(f, "  ],\n  entries: &[  \n")?;

    for entry in self.entries.iter() {
        write!(f, "{}, ", entry)?;
    }

    write!(f, "  ],\n")?;
    write!(f, "  hasher: {:?},", self.hasher)?;
    write!(f, "}};\n\n")
  }

  fn hash(&self, key: &K) -> usize {
    let mut hasher = self.hasher.build_hasher();
    key.hash(&mut hasher);
    let hash =  hasher.finish() as usize;
    if hash == 0 { 1 } else { hash }
  }
}

impl<K, V> fmt::Display for Entry<K, V>
    where K: fmt::Display, V: fmt::Display {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.key, self.value)
  }
}