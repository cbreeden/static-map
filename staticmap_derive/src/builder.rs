#![allow(dead_code)]
use std::cmp;
use std::mem;

use syn;
use quote::Tokens;
use quote::ToTokens;
use fxhash;

const MIN_TABLE_SIZE: usize = 16;

pub struct Builder {
    pub hashes: Vec<usize>,
    pub entries: Vec<Option<(syn::Lit, String)>>,
}

impl Builder {
    pub fn with_capacity(size: usize) -> Builder {
        let cap = cmp::max((size / 9 * 10).next_power_of_two(), MIN_TABLE_SIZE);

        Builder {
            hashes: vec![0; cap],
            entries: vec![None; cap],
        }
    }

    pub fn insert(&mut self, key: syn::Lit, value: String) {
        assert!(self.entries.len().is_power_of_two());

        let mask = self.entries.len() - 1;
        let mut hash = hash(&key);
        let mut entry = (key, value);

        let mut pos = hash & mask;
        let mut dist = 0;

        loop {
            if dist > self.entries.len() {
                panic!("staticmap! fatal error -- unable to find emptry bucket for key");
            }

            let probe_hash = &mut self.hashes[pos];

            // Found an empty bucket.
            if *probe_hash == 0 {
                *probe_hash = hash;
                self.entries[pos] = Some(entry);
                return
            }

            // Check if current key has an ideal dist less than held hash.
            // If so, replace current hash with held hash & update new dist
            let probe_dist = pos.wrapping_sub(*probe_hash) & mask;

            if probe_dist < dist {
                let probe_entry = self.entries[pos].as_mut().unwrap();
                mem::swap(probe_entry, &mut entry);
                mem::swap(probe_hash, &mut hash);
                dist = probe_dist;
            }

            pos = (pos + 1) & mask;
            dist += 1;
        }
    }

    pub fn build(&self, default_key: &str, default_value: &str) -> String {
        let mut result = String::new();

        result += "Map {\n    hashes: &[ ";
        for hash in &self.hashes {
            result += &format!("{}usize, ", hash);
        }

        result += "],\n    entries: &[ \n";
        for entry in &self.entries {
            match entry {
                &Some( (ref key, ref value) ) => {
                    let mut toks = Tokens::new();
                    key.to_tokens(&mut toks);
                    let key = toks.parse::<String>().unwrap();

                    result += &format!("({}, {}), ", key, value);
                },
                _ => result += &format!("({}, {}), ", default_key, default_value),
            }
        }

        result += "],\n}\n\n";
        result
    }
}

fn hash(key: &syn::Lit) -> usize {
    use syn::Lit;
    let hash = match key {
        &Lit::Str(ref s, _) => fxhash::hash(s),
        &Lit::ByteStr(ref v, _) => fxhash::hash(v),
        &Lit::Byte(n) => fxhash::hash(&n),
        &Lit::Char(c) => fxhash::hash(&c),
        &Lit::Int(n, _) => fxhash::hash(&n),
        k => {
            let err = format!("Unsupported key type: `{:?}`", k);
            panic!(err);
        }
    };

    hash as usize | 1
}