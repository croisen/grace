use logos::{Logos, skip};

use crate::api::lexers::{
    GraceError, LexerExtras, LexingError, Token, tokenize,
};

#[derive(Default, Debug, Clone, PartialEq, Logos)]
#[logos(error = LexingError, extras = LexerExtras)]
pub enum JavaTokenKind {
    StrictKeyword,

    ReservedKeyword,

    #[token(".")]
    MemberAccess,

    #[token("!")]
    #[token("++")]
    #[token("--")]
    #[token("?)")]
    UnaryOp,

    #[token("+")]
    #[token("-")]
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct JavaToken {
    pub kind: JavaTokenKind,
    pub text: String,
    pub span: (usize, usize),
    pub line: usize,
    pub col: usize,
}

impl<'a> JavaToken {
    pub fn tokenize(source: &'a str) -> (Vec<Self>, Vec<GraceError>) {
        tokenize(source)
    }
}

impl<'a> Token<'a> for JavaToken {
    #[flutter_rust_bridge::frb(ignore)]
    type TK = JavaTokenKind;

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

    fn text(&self) -> &str {
        &self.text
    }

    fn line(&self) -> usize {
        self.line
    }

    fn col(&self) -> usize {
        self.col
    }
}
