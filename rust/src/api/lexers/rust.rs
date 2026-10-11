use logos::Logos;

use crate::api::lexers::{
    GraceError, LexerExtras, LexingError, Token, tokenize,
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
    #[token("::")]
    DColon,
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
    #[token("const")]
    Const,
    #[token("pub")]
    Pub,
    #[token("trait")]
    Trait,
    #[token("impl")]
    Impl,
    #[token("in")]
    In,

    #[token("for")]
    For,
    #[token("loop")]
    Loop,
    #[token("while")]
    While,
    #[token("if")]
    If,
    #[token("else")]
    Else,

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
    #[regex(r"\n|\r\n")]
    Newline,
    #[regex(r"[ \t]+")]
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
    pub fn tokenize(source: &'a str) -> (Vec<Self>, Vec<GraceError>) {
        tokenize(source)
    }
}

impl<'a> Token<'a> for RustToken {
    #[flutter_rust_bridge::frb(ignore)]
    type TK = RustTokenKind;

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
