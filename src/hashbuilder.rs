use std::cmp;
use std::mem;
use std::marker::PhantomData;
use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;

use super::Entry;

const MIN_TABLE_SIZE: usize = 32;

pub struct Table<K, V, H> {
  mask:    usize,
  entries: Vec<Entry<K, V>>,
  _hasher: PhantomData<H>,
}

impl<K, V, H> Table<K, V, H>
    where K: Hash + Default + Eq + Debug,
          H: Hasher + Default,
          V: Default {
  pub fn with_capacity(size: u32) -> Table<K, V, H> {
    // Table size must be a power of two.
    let cap = cmp::max((size * 10/9).next_power_of_two() as usize, MIN_TABLE_SIZE);
    let mut entries = Vec::with_capacity(cap);

    for _ in 0..cap {
      entries.push(Entry::<K, V>::default());
    }

    Table {
      mask:    cap - 1,
      entries: entries,
      _hasher: PhantomData,
    }
  }

  pub fn insert(&mut self, key: K, value: V) {
    let mut entry = Entry {
      hash:  Self::hash(&key),
      key:   key,
      value: value,
    };

    // Hash the given key, determine ideal index
    let mut pos  = entry.hash & self.mask as usize;
    let mut dist = 0;

    loop {
      let probe = unsafe { self.entries.get_unchecked_mut(pos) };

      // Found an empty bucket.  Place hash and return.
      if probe.hash == 0 {
        *probe = entry;
        return
      }

      // Check if current key has an ideal dist less than held hash.
      // If so, replace current hash with held hash, update new dist
      // and continue.
      let ideal = probe.hash;
      let probe_dist = pos.wrapping_sub(ideal) & self.mask;

      if probe_dist < dist {
        mem::swap(probe, &mut entry);
        dist = probe_dist;
      }

      pos = (pos + 1) & self.mask;
      dist += 1;
    }
  }

  fn hash(key: &K) -> usize {
    let mut hasher = H::default();
    key.hash(&mut hasher);
    let hash =  hasher.finish() as usize;
    if hash == 0 { 1 } else { hash }
  }
}