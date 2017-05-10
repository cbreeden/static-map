use std::cmp;
use std::mem;
use std::default::Default;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;
use std::hash::BuildHasher;
use std::io;

const MIN_TABLE_SIZE: usize = 32;

pub struct Builder<K, V, S> {
    pub hashes: Vec<usize>,
    pub entries: Vec<(K, V)>,
    pub hasher: S,
}

impl<K, V, S> Builder<K, V, S>
    where K: Hash + Eq + Default + Debug,
          V: Default + Debug,
          S: BuildHasher + Debug
{
    pub fn with_capacity(size: usize, hasher: S) -> Builder<K, V, S> {
        // Builder size must be a power of two.
        let cap = cmp::max((size * 10 / 9).next_power_of_two(), MIN_TABLE_SIZE);
        let mut entries = Vec::with_capacity(cap);

        for _ in 0..cap {
            entries.push(<(K, V) as Default>::default());
        }

        Builder {
            hashes: vec![0; cap],
            entries: entries,
            hasher: hasher,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> usize {
        let mask = self.entries.len() - 1;
        let mut hash = self.hash(&key);
        let mut pos = hash & mask;
        let mut dist = 0;

        let mut entry = (key, value);

        loop {
            let probe_hash = unsafe { self.hashes.get_unchecked_mut(pos) };

            // Found an empty bucket.  Place hash and return.
            if *probe_hash == 0 {
                let probe = unsafe { self.entries.get_unchecked_mut(pos) };
                *probe_hash = hash;
                *probe = entry;
                return dist;
            }

            // Check if current key has an ideal dist less than held hash.
            // If so, replace current hash with held hash, update new dist
            // and continue.
            let probe_dist = pos.wrapping_sub(*probe_hash) & mask;

            if probe_dist < dist {
                let probe = unsafe { self.entries.get_unchecked_mut(pos) };
                mem::swap(probe, &mut entry);
                mem::swap(probe_hash, &mut hash);
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
            write!(f, "{:?}, ", entry)?;
        }

        write!(f, "  ],\n")?;
        write!(f, "  hasher: {:?},", self.hasher)?;
        write!(f, "}};\n\n")
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = self.hasher.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;
        if hash == 0 { 1 } else { hash }
    }
}
