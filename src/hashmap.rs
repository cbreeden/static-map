use std::hash::Hash;
use std::hash::Hasher;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Default, Debug)]
pub struct Entry<K, V> {
  pub hash:  usize,
  pub key:   K,
  pub value: V,
}

pub struct Map<K: 'static, V: 'static, H: Hasher> {
  pub entries:  &'static [Entry<K, V>],
  pub _hasher:  PhantomData<H>,
}

impl<K, V, H> Map<K, V, H>
    where K: Hash + Default + Eq + Debug,
          H: Hasher + Default {
  pub fn len(&self) -> usize {
    self.entries.len()
  }

  pub fn is_empty(&self) -> bool {
    self.entries.len() == 0
  }

  pub fn get(&self, key: &K) -> Option<&V> {
    let mask = self.len() - 1;
    let hash = Self::hash(key);
    let mut pos  = hash & mask;
    let mut dist = 0;

    loop {
      let entry = unsafe { self.entries.get_unchecked(pos) };
      if entry.hash == hash && entry.key == *key {
        return Some(&entry.value)
      }

      if entry.hash == 0 { return None }

      // Compare this to just taking dist > entry.max_dist
      let entry_dist = pos.wrapping_sub(entry.hash) & mask;
      if dist > entry_dist { return None }

      pos = (pos + 1) & mask;
      dist += 1;
    }
  }

//	pub fn keys(&self) -> Keys<K, V> {}
//  pub fn values(&self) -> Values<K, V> {}
//# pub fn values_mut(&mut self) -> ValuesMut<K, V> {}
//  pub fn iter(&self) -> Iter<K, V> {}
//# pub fn iter_mut(&mut self) -> IterMut<K, V> {}
//  pub fn entry(&mut self, key: K) -> Entry<K, V> {}
//  pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool ... {}
//# pub fn get_mut<Q: ..>(&mut self, k: &Q) -> Option<&mut V> ... {}

  #[cfg(test)]
  pub fn lookup_dist(&self, key: &K) -> usize {
    let mask = self.entries.len() - 1;
    let hash = Self::hash(key);
    let mut pos  = hash & mask;
    let mut dist = 1;

    loop {
      let current_entry = unsafe { self.entries.get_unchecked(pos) };
      if current_entry.hash == hash && self.entries[pos].key == *key {
        return dist
      } else if current_entry.hash == 0 {
        panic!("Unable to find key")
      }

      pos = (pos + 1) & mask;
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

// impl Clone for HashMap
// impl PartialEq for HashMap
// impl Eq for HashMap
// impl Index for HashMap
// impl IntoIterator for &'a HashMap