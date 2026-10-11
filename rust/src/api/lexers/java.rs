use logos::Logos;

use crate::api::lexers::{
    GraceError, LexerExtras, LexingError, Token, tokenize,
};

#[derive(Default, Debug, Clone, PartialEq, Logos)]
#[logos(error = LexingError, extras = LexerExtras)]
pub enum JavaTokenKind {
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

    #[token("abstract")]
    Abstract,
    #[token("class")]
    Class,
    #[token("continue")]
    Continue,
    #[token("const")]
    Const,
    #[token("extends")]
    Extends,
    #[token("final")]
    Final,
    #[token("goto")]
    Goto,
    #[token("implements")]
    Implements,
    #[token("import")]
    Import,
    #[token("instanceof")]
    Instanceof,
    #[token("interface")]
    Interface,
    #[token("module")]
    Module,
    #[token("native")]
    Native,
    #[token("new")]
    New,
    #[token("package")]
    Package,
    #[token("private")]
    Private,
    #[token("protected")]
    Protected,
    #[token("public")]
    Public,
    #[token("requires")]
    Requires,
    #[token("return")]
    Return,
    #[token("static")]
    Static,
    #[token("super")]
    Super,
    #[token("throw")]
    Throw,
    #[token("transient")]
    Transient,
    #[token("var")]
    Var,
    #[token("volatile")]
    Volatile,

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
}
