extern crate fxhash;
use std::hash::Hash;
use std::borrow::Borrow;

#[derive(Debug, Copy, Clone)]
pub struct Map<'a, K: 'a, V: 'a> {
    #[doc(hidden)]
    pub hashes: &'a [usize],

    #[doc(hidden)]
    pub entries: &'a [(K, V)],
}

impl<'a, K, V> Map<'a, K, V>
    where K: Hash + Eq
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
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&'a V>
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        self.get_entry(key).map(|(_, v)| v)
    }

    #[inline]
    pub fn get_entry<Q: ?Sized>(&self, key: &Q) -> Option<(&'a K, &'a V)>
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        assert!(self.entries.len().is_power_of_two(),
                "Invalid StaticMap.  The pool must be a power of two.");
        assert!(self.entries.len() >= 16,
                "Invalid StaticMap.  The pool must have size >= 16.");

        // The mask yields the number of trailing bits of a
        // hash key which yeilds its ideal position.
        let mask = self.len() - 1;
        let hash = Self::hash(key);

        let mut pos = hash & mask;
        let mut dist = 0;

        loop {
            let entry_hash = self.hashes[pos];

            // Hash match found
            if entry_hash == hash {
                let entry = &self.entries[pos];
                if key.eq(entry.0.borrow()) {
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

            pos = pos.wrapping_add(1) & mask;
            dist += 1;
        }
    }

    #[inline]
    pub fn entries(&self) -> Entries<'a, K, V> {
        Entries {
            cursor: 0,
            hashes: self.hashes,
            entries: self.entries,
        }
    }

    #[inline]
    pub fn keys(&self) -> Keys<'a, K, V> {
        Keys { entries: self.entries() }
    }

    #[inline]
    pub fn values(&self) -> Values<'a, K, V> {
        Values { entries: self.entries() }
    }

    #[inline]
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        self.get_entry(key).is_some()
    }

    #[inline]
    fn hash<Q: ?Sized>(key: &Q) -> usize
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        fxhash::hash(key) as usize | 1
    }
}

pub struct Entries<'a, K: 'a, V: 'a> {
    cursor: usize,
    hashes: &'a [usize],
    entries: &'a [(K, V)],
}

impl<'a, K: 'a, V: 'a> Iterator for Entries<'a, K, V> {
    type Item = &'a (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        while self.cursor < self.hashes.len() {
            if self.hashes[self.cursor] != 0 {
                let result = Some(&self.entries[self.cursor]);
                self.cursor += 1;
                return result;
            }
            self.cursor += 1
        }

        None
    }
}

pub struct Keys<'a, K: 'a, V: 'a> {
    entries: Entries<'a, K, V>,
}

impl<'a, K: 'a, V: 'a> Iterator for Keys<'a, K, V> {
    type Item = &'a K;
    fn next(&mut self) -> Option<Self::Item> {
        self.entries.next().map(|entry| &entry.0)
    }
}

pub struct Values<'a, K: 'a, V: 'a> {
    entries: Entries<'a, K, V>,
}

impl<'a, K: 'a, V: 'a> Iterator for Values<'a, K, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        self.entries.next().map(|entry| &entry.1)
    }
}

impl<'a, K: 'a, V: 'a> IntoIterator for Map<'a, K, V>
    where K: Hash + Eq
{
    type Item = &'a (K, V);
    type IntoIter = Entries<'a, K, V>;
    fn into_iter(self) -> Entries<'a, K, V> {
        self.entries()
    }
}

#[macro_export]
macro_rules! static_map {
    (Default: $default:expr, $($key:expr => $value:expr),* $(,)*) => (
        static_map!(@stringify ($default) $( ($key ($value)) )*)
    );

    // This trick for stringifying into a procedural macro was
    // developed by dtonley in the proc-macro-hack crate.
    (@stringify $($tt:tt)*) => ({
        #[derive(StaticMapMacro)]
        enum __StaticMap__ {
            A = static_map!(@zero $($tt)*)
        }

        __static_map__construct_map!()
    });

    (@zero $($tt:tt)*) => { 0 }
}