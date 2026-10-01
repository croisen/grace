use anyhow::Result;
use logos::Logos;

use crate::utils::lexer::{
    LexerExtras, LexingError, Token, extras_newline, tokenize,
};

#[derive(Default, Debug, Clone, PartialEq, Logos)]
#[logos(error = LexingError, extras = LexerExtras)]
pub enum RustTokenKind {
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
    #[token("(")]
    OParen,
    #[token(")")]
    CParen,
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
    #[token("?")]
    Question,

    #[token("use")]
    Use,
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("->")]
    Return,
    #[token("static")]
    Static,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*!")]
    Macro,
    #[regex(r"'[a-zA-Z_][a-zA-Z0-9_]*")]
    Lifetime,

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
    #[regex("-?[0-9]{1,3}(_[0-9]{3})*")]
    Integer,
    #[regex(r#""([^"\\]|\\.)""#)]
    String,
    #[regex(r"\n|\r\n", extras_newline)]
    Newline,
    #[regex(r"[ \t]+", logos::skip)]
    Whitespace,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RustToken {
    pub kind: RustTokenKind,
    pub text: String,
    pub span: (usize, usize),
    pub line: usize,
    pub col: usize,
}

impl<'a> RustToken {
    pub fn tokenize(source: &'a str) -> Result<Vec<Self>> {
        tokenize::<RustToken, RustTokenKind>(source)
    }
}

impl Token<RustTokenKind> for RustToken {
    fn nt(
        kind: RustTokenKind,
        source: &str,
        span: (usize, usize),
        extras: LexerExtras,
    ) -> Self {
        Self {
            kind,
            text: source[span.0..span.1].to_string(),
            span,
            line: extras.line,
            col: extras.col,
        }
    }
}
