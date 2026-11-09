use logos::{Logos, skip};

use crate::core::errors::kind::GraceErrorKind;
use crate::core::lexers::{LexerExtras, Token, reserved_keyword};

pub type JavaToken<'a> = Token<'a, JavaTokenKind>;

#[derive(Default, Debug, Clone, PartialEq, Logos)]
#[logos(error = GraceErrorKind, extras = LexerExtras)]
pub enum JavaTokenKind {
    #[token("@")]
    #[token("abstract")]
    #[token("assert")]
    #[token("break")]
    #[token("case")]
    #[token("catch")]
    #[token("class")]
    #[token("continue")]
    #[token("default")]
    #[token("do")]
    #[token("else")]
    #[token("enum")]
    #[token("extends")]
    #[token("final")]
    #[token("finally")]
    #[token("for")]
    #[token("if")]
    #[token("implements")]
    #[token("import")]
    #[token("instanceof")]
    #[token("interface")]
    #[token("native")]
    #[token("new")]
    #[token("package")]
    #[token("permits")]
    #[token("private")]
    #[token("protected")]
    #[token("public")]
    #[token("record")]
    #[token("return")]
    #[token("sealed")]
    #[token("static")]
    #[token("strictfp")]
    #[token("super")]
    #[token("switch")]
    #[token("synchronized")]
    #[token("this")]
    #[token("throw")]
    #[token("throws")]
    #[token("transient")]
    #[token("try")]
    #[token("volatile")]
    #[token("while")]
    StrictKeyword,

    #[token("const", reserved_keyword)]
    #[token("goto", reserved_keyword)]
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
    Identifier,

    #[regex("[+-]?[0-9][_0-9]*")]
    #[regex("[+-]?0[bB]_*[01][_01]*")]
    #[regex("[+-]?0[oO]_*[0-7][_0-7]*")]
    #[regex("[+-]?0[xX]_*[0-9a-fA-F][_0-9a-fA-F]*")]
    #[default]
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
