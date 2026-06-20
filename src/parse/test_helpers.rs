use camino::Utf8PathBuf;

use super::{lexer::tokenize, parse_module};

pub(crate) fn parse(src: &str) -> String {
    let module = parse_module(Utf8PathBuf::from("test/module.gleam"), src).expect("should parse");
    format!("{module:#?}")
}

pub(crate) fn reject(src: &str) -> String {
    let error =
        parse_module(Utf8PathBuf::from("test/module.gleam"), src).expect_err("should not parse");
    format!("{error:#?}")
}

pub(crate) fn lex(src: &str) -> String {
    let tokens = tokenize(src).expect("should lex");
    tokens
        .iter()
        .map(|token| format!("{:?} @ {}..{}", token.token, token.start, token.end))
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn lex_reject(src: &str) -> String {
    let error = tokenize(src).expect_err("should not lex");
    format!("{error:#?}")
}
