use std::cmp;
use std::default::Default;
use std::fmt::Debug;
use std::hash::{BuildHasher, Hash, Hasher};
use std::io;
use std::mem;

const MIN_TABLE_SIZE: usize = 32;

pub struct Builder<K, V, S> {
    pub hashes: Vec<usize>,
    pub entries: Vec<(K, V)>,
    pub hasher: S,
}

impl<K, V, S> Builder<K, V, S>
    where K: Hash + Eq + Default + Debug,
          V: Default + Debug,
          S: Hasher + Debug + Clone
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
        // The mask yields the number of trailing bits of a
        // hash key which yeilds its ideal position.
        let mask = self.entries.len() - 1;
        let mut hash = self.hash(&key);
        let mut entry = (key, value);

        let mut pos = hash & mask;
        let mut dist = 0;

        loop {
            if dist > self.entries.len() {
                panic!("Something wen't wrong.  Unable to find empty bucket.");
            }

            let probe_hash = &mut self.hashes[pos];

            // Found an empty bucket.  Place hash and return.
            if *probe_hash == 0 {
                *probe_hash = hash;
                self.entries[pos] = entry;
                return dist;
            }

            // Check if current key has an ideal dist less than held hash.
            // If so, replace current hash with held hash, update new dist
            // and continue.
            let probe_dist = pos.wrapping_sub(*probe_hash) & mask;

            if probe_dist < dist {
                let probe_entry = &mut self.entries[pos];
                mem::swap(probe_entry, &mut entry);
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
        let mut hasher = self.hasher.clone();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;
        hash | 1
    }
}