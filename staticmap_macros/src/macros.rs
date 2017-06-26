macro_rules! expect_delimited {
    ($token:expr) => (
        match *$token {
            TokenTree::Delimited(Delimited { ref tts, .. }) => tts,
            _ => panic!("fatal: expected a delimited, found a single token"),
        }
    )
}

macro_rules! expect_lit {
    ($token:expr) => (
        match *$token {
            TokenTree::Token(Token::Literal(ref lit)) => lit,
            ref x => panic!("fatal: expected a literal, found `{:?}`", x),
        }
    )
}