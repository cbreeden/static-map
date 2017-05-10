use std::hash::Hash;
use std::hash::Hasher;
use std::hash::BuildHasher;

pub struct Map<K: 'static, V: 'static, S: BuildHasher> {
    #[doc(hidden)]
    pub hasher: S,

    #[doc(hidden)]
    pub hashes: &'static [usize],

    #[doc(hidden)]
    pub entries: &'static [(K, V)],
}

impl<K, V, S> Map<K, V, S>
    where K: Hash + Eq,
          S: BuildHasher
{
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.len() == 0
    }

    #[inline]
    pub fn get(&self, key: &K) -> Option<&'static V> {
        self.get_entry(key).map(|(_, v)| v)
    }

    #[inline]
    pub fn get_entry(&self, key: &K) -> Option<(&'static K, &'static V)> {
        assert!(self.entries.len().is_power_of_two(),
                "Invalid StaticMap.  The pool must be a power of two.");
        assert!(self.entries.len() >= 32,
                "Invalid StaticMap.  The pool must have size >= 32.");

        // The mask yields the number of trailing bits of a
        // hash key which yeilds its ideal position.
        let mask = self.len() - 1;
        let hash = self.hash(key);

        let mut pos = hash & mask;
        let mut dist = 0;

        loop {
            let entry_hash = self.hashes[pos];

            // Hash match found
            if entry_hash == hash {
                let entry = &self.entries[pos];
                if entry.0 == *key {
                    return Some((&entry.0, &entry.1));
                }
            }

            // A zero hash indicates vacent bin
            if entry_hash == 0 {
                return None;
            }

            // The trailing bits of a hash indicates the position
            // the hash is stored.  Taking the difference between
            // the current hash value and the position will yield
            // the distance from the current key and it's ideal
            // location.  If the current key's distance from ideal
            // is greater than distance from our key's ideal then
            // we know there is no match.  This is the key
            // characteristic of Robin-Hood hashing.
            let entry_dist = pos.wrapping_sub(entry_hash) & mask;
            if dist > entry_dist {
                return None;
            }

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

    fn hash(&self, key: &K) -> usize {
        let mut hasher = self.hasher.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;
        hash | 1
    }
}

// impl Clone for HashMap
// impl PartialEq for HashMap
// impl Eq for HashMap
// impl Index for HashMap
// impl IntoIterator for &'a HashMap
