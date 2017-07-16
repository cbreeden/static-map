#![allow(dead_code)]
use std::cmp;
use std::mem;

use fxhash;
use quote;
use syn;

use Key;
use Value;

const MIN_TABLE_SIZE: usize = 16;

pub struct Builder<'a> {
    pub hashes: Vec<usize>,
    pub entries: Vec<Option<(Key<'a>, Value<'a>)>>,
}

macro_rules! tok_lit {
    ($lit:expr) => ({
        let mut t = quote::Tokens::new();
        t.append($lit);
        t
    })
}

impl<'a> Builder<'a> {
    pub fn with_capacity(size: usize) -> Builder<'a> {
        let cap = cmp::max((size / 9 * 10).next_power_of_two(), MIN_TABLE_SIZE);

        Builder {
            hashes: vec![0; cap],
            entries: vec![None; cap],
        }
    }

    pub fn insert(&mut self, key: Key<'a>, value: Value<'a>) {
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
                return;
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

            pos = pos.wrapping_add(1) & mask;
            dist += 1;
        }
    }

    pub fn build(self, default_key: Key, default_value: Value) -> quote::Tokens {
        let hashes = self.hashes;
        let entries = self.entries
            .into_iter()
            .map(|opt| match opt {
                     Some(opt) => (opt.0, tok_lit!(opt.1)),
                     None => (default_key.clone(), tok_lit!(default_value)),
                 });

        quote! {
            static_map::Map {
                hashes: &[ #(#hashes),* ],
                entries: &[ #(#entries),* ],
            }
        }
    }
}

fn hash(key: &syn::Lit) -> usize {
    use syn::Lit;
    let hash = match *key {
        Lit::Str(ref s, _) => _hash(s),
        Lit::ByteStr(ref v, _) => _hash(v),
        Lit::Byte(n) => _hash(&n),
        Lit::Char(c) => _hash(&c),
        Lit::Int(n, _) => _hash(&n),
        ref k => {
            let err = format!("Unsupported key type: `{:?}`", k);
            panic!(err);
        }
    };

    hash
}

use std::hash::Hash;
fn _hash<Q: ?Sized>(key: &Q) -> usize
    where Q: Hash + Eq
{
    fxhash::hash(key) as usize | 1
}