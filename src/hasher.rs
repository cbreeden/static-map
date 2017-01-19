use std::cmp;
use std::hash::Hash;
use std::hash::Hasher;
use std::fmt::Debug;
use std::mem;

use std::marker::PhantomData;

const MIN_TABLE_SIZE: usize = 32;

#[derive(Default, Debug)]
struct Entry<K> {
  hash: usize,
  key:  K,
}

pub struct Table<K, H> {
  mask:     usize,
  entries:  Vec<Entry<K>>,
  _hasher:   PhantomData<H>,
}

impl<K, H> Table<K, H>
    where K: Hash + Default + Eq + Debug,
          H: Hasher + Default {
  pub fn with_capacity(size: u32) -> Table<K, H> {
    // Table size must be a power of two.
    let cap = cmp::max((size * 10/9).next_power_of_two() as usize, MIN_TABLE_SIZE);
    let mut entries = Vec::with_capacity(cap);

    for _ in 0..cap {
      entries.push(Entry::<K>::default());
    }

    Table {
      mask:    cap - 1,
      entries: entries,
      _hasher:  PhantomData,
    }
  }

  pub fn insert(&mut self, mut key: K) {
    // Hash the given key, determine ideal index
    let mut hash = Self::hash(&key);
    let mut pos  = hash & self.mask as usize;
    let mut dist = 0;

    loop {
      let current_entry = unsafe { self.entries.get_unchecked_mut(pos) };

      // Found an empty bucket.  Place hash and return.
      if current_entry.hash == 0 {
        current_entry.hash = hash;
        current_entry.key =  key;
        return
      }

      // Check if current key has an ideal dist less than held hash.
      // If so, replace current hash with held hash, update new dist
      // and continue.
      let ideal = current_entry.hash;
      let current_dist = pos.wrapping_sub(ideal) & self.mask;

      if current_dist < dist {
        mem::swap(&mut key, &mut current_entry.key);
        mem::swap(&mut hash, &mut current_entry.hash);
        dist = current_dist;
      }

      pos = (pos + 1) & self.mask;
      dist += 1;
    }
  }

  pub fn lookup_index(&self, key: &K) -> Option<(usize, usize)> {
    let hash = Self::hash(key);
    let mut pos  = hash & self.mask;
    let mut dist = 1;

    loop {
      let current_entry = unsafe { self.entries.get_unchecked(pos) };
      if current_entry.hash == hash && self.entries[pos].key == *key {
        return Some((pos, dist))
      } else if current_entry.hash == 0 {
        return None
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
