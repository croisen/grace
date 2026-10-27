use logos::{Logos, skip};

use crate::api::lexers::{LexerExtras, LexingError, Token};

pub(crate) type SQLToken<'a> = Token<'a, SQLTokenKind>;

#[derive(Default, Debug, Clone, PartialEq, Logos)]
#[logos(error = LexingError, extras = LexerExtras)]
pub(crate) enum SQLTokenKind {
    StrictKeyword,

    ReservedKeyword,

    #[token("+")]
    #[token("-")]
    UBinaryOp,

    #[token("*")]
    #[token("/")]
    #[token("<<")]
    #[token(">>")]
    #[token("<")]
    #[token("<=")]
    #[token(">=")]
    #[token(">")]
    #[token("==")]
    #[token("!=")]
    #[token("&")]
    #[token("|")]
    #[token("^")]
    #[token("&&")]
    #[token("||")]
    #[token("+=")]
    #[token("-=")]
    #[token("*=")]
    #[token("/=")]
    #[token("%=")]
    #[token("<<=")]
    #[token("=>>")]
    #[regex("=")]
    BinaryOp,

    #[token("(")]
    #[token(")")]
    #[token("{")]
    #[token("}")]
    #[token("[")]
    #[token("]")]
    Pairs,

    #[token(";")]
    #[token(",")]
    Delimiters,

    #[token(":")]
    Colon,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    #[default]
    Identifier,

    #[regex("[+-]?[0-9][_0-9]*")]
    #[regex("[+-]?0[bB]_*[01][_01]*")]
    #[regex("[+-]?0[oO]_*[0-7][_0-7]*")]
    #[regex("[+-]?0[xX]_*[0-9a-fA-F][_0-9a-fA-F]*")]
    Integer,

    #[regex(r#"[+-]?(([0-9][_0-9]*\.([0-9][_0-9]*)?([eE][+-]?([0-9][_0-9]*))?[fFdD]?)|(\.([0-9][_0-9]*)([eE][+-]?([0-9][_0-9]*))?[fFdD]?)|(([0-9][_0-9]*)([eE][+-]?([0-9][_0-9]*))[fFdD]?)|(([0-9][_0-9]*)([eE][+-]?([0-9][_0-9]*))?[fFdD]))"#)]
    Float,

    #[regex(r#""([^"\\]|\\.)""#)]
    String,

    #[regex(r"//[^\n]*", allow_greedy = true)]
    #[regex(r"/\*(?:[^*]\*[^/])*\*/")]
    Comment,

    #[regex(r"\n|\r\n")]
    Newline,
    #[regex(r"[ \t]+", skip)]
    Whitespace,
}
