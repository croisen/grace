use logos::{Logos, skip};

use crate::api::lexers::{
    GraceError, LexerExtras, LexingError, Token, reserved_keyword, tokenize,
};

#[derive(Default, Debug, Clone, PartialEq, Logos)]
#[logos(error = LexingError, extras = LexerExtras)]
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
