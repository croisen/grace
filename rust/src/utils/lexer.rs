use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use anyhow::Result;
use logos::{Lexer, Logos, Skip};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexerExtras {
    pub line: usize,
    pub col: usize,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    #[default]
    Unknown,
}

impl Display for LexingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{self:?}")
    }
}

impl Error for LexingError {}

pub trait Token<TokenKind> {
    fn nt(
        kind: TokenKind,
        source: &str,
        span: (usize, usize),
        extras: LexerExtras,
    ) -> Self;
}

#[flutter_rust_bridge::frb(ignore)]
pub fn extras_newline<'a, T>(lex: &mut Lexer<'a, T>) -> Skip
where
    T: Logos<'a, Source = str, Error = LexingError, Extras = LexerExtras>,
{
    lex.extras.line += 1;
    lex.extras.col = lex.span().end;
    Skip
}

#[flutter_rust_bridge::frb(ignore)]
pub fn tokenize<'a, T, TK>(source: &'a str) -> Result<Vec<T>>
where
    T: Token<TK>,
    TK: Logos<'a, Source = str, Error = LexingError, Extras = LexerExtras>
        + 'static,
{
    let mut lexer = TK::lexer(source);
    lexer.extras = LexerExtras { line: 1, col: 0 };
    let mut tokens = vec![];
    while let Some(t) = lexer.next() {
        let (token, span) = (t?, lexer.span());
        lexer.extras.col = span.start - lexer.extras.col + 1;
        tokens.push(T::nt(token, source, (span.start, span.end), lexer.extras))
    }

    Ok(tokens)
}
