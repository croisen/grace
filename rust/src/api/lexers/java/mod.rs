use logos::Logos;

use crate::api::lexers::{extras_newline, LexingError};

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(error = LexingError, extras = (usize, usize))]
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
