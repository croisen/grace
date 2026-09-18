use std::error::Error;
use std::fmt::{Display, Error as FError, Formatter};
use std::num::ParseIntError;
use std::ops::Range;

use anyhow::Result;
use logos::{Lexer, Logos, Skip};

pub mod java;
pub mod rust;
pub mod sql;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    NumberParseError,
    #[default]
    Other,
}

impl Display for LexingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FError> {
        write!(f, "Lexing Error: {self:?}")
    }
}

impl Error for LexingError {}

impl From<ParseIntError> for LexingError {
    fn from(_: ParseIntError) -> Self {
        Self::NumberParseError
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a, T>
where
    T: Logos<'a, Source = str, Error = LexingError, Extras = (usize, usize)>,
{
    kind: T,
    text: &'a str,
    span: Range<usize>,
    line: usize,
    col: usize,
}

impl<'a, T> Token<'a, T>
where
    T: Logos<'a, Source = str, Error = LexingError, Extras = (usize, usize)>,
{
    pub fn new(kind: T, source: &'a str, span: Range<usize>, line: usize, col: usize) -> Self {
        Self {
            kind,
            text: &source[span.clone()],
            span,
            line,
            col,
        }
    }

    pub fn tokenize(source: &'a str) -> Result<Vec<Self>> {
        let mut lexer = T::lexer(source);
        lexer.extras = (1, 0);
        let mut tokens = vec![];
        while let Some(t) = lexer.next() {
            let (token, span) = (t?, lexer.span());
            let line = lexer.extras.0;
            let col = span.start - lexer.extras.1 + 1;
            tokens.push(Token::new(token, source, span, line, col))
        }

        Ok(tokens)
    }
}

fn extras_newline<'a, T>(lex: &mut Lexer<'a, T>) -> Skip
where
    T: Logos<'a, Extras = (usize, usize)>,
{
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    Skip
}
