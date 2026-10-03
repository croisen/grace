use anyhow::Result;
use logos::Logos;

use crate::api::lexers::{GraceError, LexerExtras, LexingError};
use crate::utils::lexer::{Token, extras_newline, tokenize};

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
    #[regex(r"\n|\r\n", extras_newline)]
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
    pub fn tokenize(source: &'a str) -> Result<Vec<Self>, Vec<GraceError>> {
        tokenize::<JavaToken, JavaTokenKind>(source)
    }
}

impl Token<JavaTokenKind> for JavaToken {
    fn nt(
        kind: JavaTokenKind,
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
