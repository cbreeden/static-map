#![feature(proc_macro)]
extern crate proc_macro;
extern crate fxhash;
extern crate syn;
extern crate itoa;
extern crate quote;

mod builder;

use std::str::FromStr;
use builder::Builder;
use proc_macro::TokenStream;
use syn::parse::IResult;
use syn::parse;

#[proc_macro]
pub fn static_map_macro(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    build_static_map(&s)
}

fn build_static_map(input: &str) -> TokenStream {
    let (mut input, default_value) = extract_default(input);

    let mut key_values: Vec<(syn::Lit, String)> = Vec::new();
    while let Some(tup) = extract_key_value(input) {
        input = tup.0;
        key_values.push((tup.1, tup.2));
    }

    let mut builder = Builder::with_capacity(key_values.len());

    let default_key = {
        let default_key = key_values.get(0)
            .expect("staticmap! requires at least one key/value pair");
        lit_default(&default_key.0)
    };

    for (key, value) in key_values {
        builder.insert(key, value);
    }

    let result = builder.build(default_key, &default_value);
    // panic!(result);
    TokenStream::from_str(&result).unwrap()
}

fn extract_key_value(s: &str) -> Option<(&str, syn::Lit, String)> {
    let (s, lit) =
        match parse::lit(s) {
            IResult::Done(i, o) => (i, o),
            _ => return None,
        };

    let mut split= s.splitn(3, '@').skip(1);
    let value = split.next()
        .expect("staticmap! syntax error -- unable to find value associated to key")
        .to_owned();

    let rest = split.next().unwrap_or("");
    Some((rest, lit, value))
}

fn extract_default(s: &str) -> (&str, String) {
    let mut split = s.splitn(3, '@');

    let default = split.next().map(|v| v.trim());
    if default != Some("Default :") {
        panic!("staticmap! requires a `Default: <expr>,` for a default value");
    }

    let value = split.next()
        .expect("staticmap! insufficient content provided")
        .to_string();

    let rest = split.next()
        .expect("staticmap! requires at least one key/value pair");

    (rest, value)
}

fn lit_default(lit: &syn::Lit) -> &'static str {
    use syn::Lit::*;
    match *lit {
        Str(_, _) => r#""""#,
        ByteStr(_, _) => "[]",
        Byte(_) => "0u8",
        Char(_) => "0 as char",
        Int(_, ty) => {
            use syn::IntTy::*;
            match ty {
                Isize => "0isize",
                I8 => "0i8",
                I16 => "0i16",
                I32 => "0i32",
                I64 => "0i64",
                Usize => "0usize",
                U8 => "0u8",
                U16 => "0u16",
                U32 => "0u32",
                U64 => "0u64",
                Unsuffixed => "0",
            }
        },
        _ => panic!("staticmap! unsupported key type"),
    }
}