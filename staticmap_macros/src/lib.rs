extern crate fxhash;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::Delimited;
use syn::Token;
use syn::TokenTree;

#[macro_use]
mod macros;
mod builder;
use builder::Builder;

type Key<'a> = &'a syn::Lit;
type Value<'a> = &'a syn::TokenTree;

const LEADING: &str = "enum __StaticMap__ {\n    A =\n        static_map!(@ zero ";
const TRAILING: &str = "),\n}";

fn trim(input: &str) -> &str {
    assert!(input.starts_with(LEADING));
    assert!(input.ends_with(TRAILING));

    let (_, input) = input.split_at(LEADING.len());
    let (input, _) = input.split_at(input.len() - TRAILING.len());
    input
}

#[proc_macro_derive(StaticMapMacro)]
pub fn static_map_macro(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let result = build_static_map(trim(&s));

    let wrapper = quote! {
        macro_rules! __static_map__construct_map {
            () => ( #result )
        }
    };

    wrapper.parse().unwrap()
}

fn build_static_map(input: &str) -> quote::Tokens {
    let tt = syn::parse_token_trees(input).unwrap();

    // Extract the defualt value, which should be first token listed
    let default_value = &tt[0];
    let tt = &tt[1..];

    // (Key (Value)) pairs follow
    let mut builder = Builder::with_capacity(tt.len());
    for pair in tt {
        let pair  = expect_delimited!(pair);
        let key   = expect_lit!(&pair[0]);
        let value = &pair[1];
        builder.insert(key, value);
    }

    // Determine the default value for hashmap keys
    // TODO: Here we might be able to determine default
    // values for certain values.
    let default_key = match tt.get(0) {
        Some(pair) => {
            let pair = expect_delimited!(pair);
            let key  = expect_lit!(&pair[0]);
            lit_default(key)
        }
        _ => panic!("staticmap! requires at least one key/value pair"),
    };

    builder.build(&default_key, default_value)
}

fn lit_default(lit: &syn::Lit) -> syn::Lit {
    use syn::Lit::*;
    use syn::Lit;

    match *lit {
        Str(_, _) => Lit::from(""),
        // ByteStr(_, _) => "[]",
        Byte(_) => Lit::from(0u8),
        Char(_) => Lit::from(0 as char),
        Int(_, ty) => {
            use syn::IntTy::*;
            use syn::IntTy;
            match ty {
                Isize => Lit::from(0isize),
                I8 => Lit::from(0i8),
                I16 => Lit::from(0i16),
                I32 => Lit::from(0i32),
                I64 => Lit::from(0i64),
                Usize => Lit::from(0usize),
                U8 => Lit::from(0u8),
                U16 => Lit::from(0u16),
                U32 => Lit::from(0u32),
                U64 => Lit::from(0u64),
                Unsuffixed => Lit::Int(0, IntTy::Unsuffixed),
            }
        },
        ref lit => panic!("staticmap! unsupported key type `{:?}`", lit),
    }
}