use logos::Logos;

use crate::api::lexers::{extras_newline, LexingError};

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(error = LexingError, extras = (usize, usize))]
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
    OBrace,
    #[token("}")]
    CBrace,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex("-?[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),
    #[regex(r#""([^"\\]|\\.)""#)]
    String,
    #[regex(r"\n|\r\n", extras_newline)]
    Newline,
    #[regex(r"[ \t]+", logos::skip)]
    Whitespace,
}
