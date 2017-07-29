extern crate fxhash;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

mod builder;
use builder::Builder;

type Key<'a> = syn::Lit;
type Value<'a> = &'a str;

const LEADING: &str = "enum __StaticMap__ {\n    A =\n        static_map!(@ zero";
const TRAILING: &str = "),\n}";

fn trim(input: &str) -> &str {
    assert!(input.starts_with(LEADING));
    assert!(input.ends_with(TRAILING));

    let (_, input) = input.split_at(LEADING.len());
    let (input, _) = input.split_at(input.len() - TRAILING.len());
    input.trim()
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
    // panic!("{}", wrapper.to_string());
}

fn build_static_map(input: &str) -> quote::Tokens {
    // TODO: Splitting by @ and ? like this is not hygenic.
    //  This could fail if a value is "This @ is a ? value";
    let mut tt = input.split('@');

    // Extract the defualt value, which should be first token listed.
    // `.split()` always returns at least an empty string for the first iteration.
    let default_value = tt.next().unwrap();
    
    // "@ Key ? Value" pairs follow
    let count = input.chars().filter(|&c| c == '@').count();
    let mut builder = Builder::with_capacity(count);

    // Determine the default key from the first key-value piar
    let mut pair = tt
        .next()
        .expect("staticmap! requires at least one key/value pair")
        .split('?');

    let default_key = syn::parse::lit(pair.next().unwrap()).expect("failed to parse key type");
    let value = pair.next().unwrap();
    builder.insert(default_key.clone(), value);

    for pair in tt {
        let mut pair = pair.split('?');
        
        // Each pair is gauranteed to have a `?`.  So unwrapping is safe.
        let key = syn::parse::lit(pair.next().unwrap()).expect("failed to parse key type");
        let value = pair.next().unwrap();
        builder.insert(key, value);
    }

    builder.build(lit_default(&default_key), default_value)
}

fn lit_default(lit: &syn::Lit) -> syn::Lit {
    use syn::Lit::*;
    use syn::Lit;

    match *lit {
        Str(_, _) => Lit::from(""),
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