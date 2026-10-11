use logos::Logos;

use crate::api::lexers::{
    GraceError, LexerExtras, LexingError, Token, tokenize,
};

#[derive(Default, Debug, Clone, PartialEq, Logos)]
#[logos(error = LexingError, extras = LexerExtras)]
pub enum SQLTokenKind {
    #[token("(")]
    OParen,
    #[token(")")]
    CParen,
    #[token("`")]
    Tilde,
    #[token("=")]
    Equals,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    FSlash,
    #[token("\\")]
    BSlash,
    #[token("{")]
    OCurly,
    #[token("}")]
    CCurly,
    #[token("[")]
    OBrace,
    #[token("]")]
    CBrace,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,

    #[token(">")]
    CompGT,
    #[token("<")]
    CompLT,
    #[token(">=")]
    CompGE,
    #[token("<=")]
    CompLE,
    #[token("==")]
    CompEQ,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    #[default]
    Identifier,
    #[regex("-?[0-9]+")]
    Integer,
    #[regex(r#""([^"\\]|\\.)""#)]
    String,
    #[regex(r"\n|\r\n")]
    Newline,
    #[regex(r"[ \t]+", logos::skip)]
    Whitespace,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SQLToken {
    pub kind: SQLTokenKind,
    pub text: String,
    pub span: (usize, usize),
    pub line: usize,
    pub col: usize,
}

impl<'a> SQLToken {
    pub fn tokenize(source: &'a str) -> (Vec<Self>, Vec<GraceError>) {
        tokenize(source)
    }
}

impl<'a> Token<'a> for SQLToken {
    #[flutter_rust_bridge::frb(ignore)]
    type TK = SQLTokenKind;

    #[flutter_rust_bridge::frb(ignore)]
    fn new_token(
        kind: Self::TK,
        source: &'a str,
        span: (usize, usize),
        line_col: (usize, usize),
    ) -> Self {
        Self {
            kind,
            text: source[span.0..span.1].into(),
            span,
            line: line_col.0,
            col: line_col.1,
        }
    }
}
