use logos::{Lexer, Logos, skip};

use crate::core::errors::kind::GraceErrorKind;
use crate::core::lexers::{LexerExtras, Token, reserved_keyword};

pub type RustToken<'a> = Token<'a, RustTokenKind>;

#[derive(Default, Debug, Clone, PartialEq, Logos)]
#[logos(error = GraceErrorKind, extras = LexerExtras)]
pub enum RustTokenKind {
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*!")]
    Macro,

    #[token("'static")]
    StaticLifetime,
    #[regex(r"'[a-zA-Z_][a-zA-Z0-9_]*")]
    Lifetime,

    #[token("as")]
    #[token("async")]
    #[token("await")]
    #[token("break")]
    #[token("const")]
    #[token("continue")]
    #[token("crate")]
    #[token("dyn")]
    #[token("else")]
    #[token("enum")]
    #[token("extern")]
    #[token("false")]
    #[token("fn")]
    #[token("for")]
    #[token("if")]
    #[token("impl")]
    #[token("in")]
    #[token("let")]
    #[token("loop")]
    #[token("match")]
    #[token("mod")]
    #[token("move")]
    #[token("mut")]
    #[token("pub")]
    #[token("ref")]
    #[token("return")]
    #[token("self")]
    #[token("Self")]
    #[token("static")]
    #[token("struct")]
    #[token("super")]
    #[token("trait")]
    #[token("true")]
    #[token("type")]
    #[token("unsafe")]
    #[token("use")]
    #[token("where")]
    #[token("while")]
    #[token("macro_rules")]
    #[token("safe")]
    #[token("union")]
    StrictKeyword,

    #[token("abstract", reserved_keyword)]
    #[token("become", reserved_keyword)]
    #[token("box", reserved_keyword)]
    #[token("do", reserved_keyword)]
    #[token("final", reserved_keyword)]
    #[token("gen", reserved_keyword)]
    #[token("macro", reserved_keyword)]
    #[token("override", reserved_keyword)]
    #[token("priv", reserved_keyword)]
    #[token("try", reserved_keyword)]
    #[token("typeof", reserved_keyword)]
    #[token("unsized", reserved_keyword)]
    #[token("virtual", reserved_keyword)]
    #[token("yield", reserved_keyword)]
    #[token("++", reserved_keyword)]
    #[token("--", reserved_keyword)]
    ReservedKeyword,

    #[token("::")]
    #[token(".")]
    MemberAccess,

    #[token("!")]
    #[token("?)")]
    UnaryOp,

    #[token("&")]
    #[token("*")]
    UBinaryOp,

    #[token("+")]
    #[token("-")]
    #[token("/")]
    #[token("<<")]
    #[token(">>")]
    #[token("<")]
    #[token("<=")]
    #[token(">=")]
    #[token(">")]
    #[token("==")]
    #[token("!=")]
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

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    // well let's allow keywords to become identifiers
    #[regex("r#[a-zA-Z]*", keyword_to_ident)]
    Identifier(String),

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

    #[default]
    Unknown,
}

// extract the text after r#
fn keyword_to_ident<'a, T>(
    lex: &mut Lexer<'a, T>,
) -> Result<String, GraceErrorKind>
where
    T: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>,
{
    Ok(lex.slice()[2..].into())
}
