use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::num::{ParseFloatError, ParseIntError};

use logos::{Lexer, Logos};

use crate::api::GraceError;

pub mod java;
pub mod rust;
pub mod sql;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexerExtras {
    pub line: usize,
    pub col: usize,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    FloatParseError(String),
    IntegerParseError(String),
    ReservedKeyword(String),
    SyntaxError(String),
    UnmatchedPair(String),
    UnexpectedToken(String),
    #[default]
    Unknown,
}

impl Display for LexingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{self:?}")
    }
}

impl Error for LexingError {}

impl From<ParseIntError> for LexingError {
    fn from(v: ParseIntError) -> Self {
        Self::IntegerParseError(format!("{:?}", v.kind()))
    }
}

impl From<ParseFloatError> for LexingError {
    fn from(v: ParseFloatError) -> Self {
        Self::FloatParseError(format!("{v:?}"))
    }
}

#[flutter_rust_bridge::frb(ignore)]
pub trait Token<'a> {
    #[flutter_rust_bridge::frb(ignore)]
    type TK;
    #[flutter_rust_bridge::frb(ignore)]
    fn new_token(
        kind: Self::TK,
        source: &'a str,
        span: (usize, usize),
        line_col: (usize, usize),
    ) -> Self;

    #[flutter_rust_bridge::frb(ignore)]
    fn text(&self) -> &str;
    #[flutter_rust_bridge::frb(ignore)]
    fn line(&self) -> usize;
    #[flutter_rust_bridge::frb(ignore)]
    fn col(&self) -> usize;
}

#[flutter_rust_bridge::frb(ignore)]
fn tokenize<'a, T, TK>(source: &'a str) -> (Vec<T>, Vec<GraceError>)
where
    T: Token<'a, TK = TK> + Clone,
    TK: Logos<'a, Source = str, Error = LexingError, Extras = LexerExtras>
        + Default,
{
    let mut lexer = TK::lexer(source);
    lexer.extras = LexerExtras { line: 1, col: 0 };
    let mut tokens = vec![];
    let mut errors = vec![];
    let mut pairs = vec![];

    let p = HashMap::from([
        ("(", ")"),
        ("[", "]"),
        ("{", "}"),
        (")", "("),
        ("]", "["),
        ("}", "{"),
    ]);

    while let Some(t) = lexer.next() {
        let span = lexer.span();
        let span = (span.start, span.end);
        let text = lexer.slice();

        let LexerExtras { line, mut col } = lexer.extras;
        col = span.0 - col + 1;
        let line_col = (line, col);
        match t {
            Ok(token) => {
                let t = T::new_token(token, source, span, line_col);
                if matches!(text, "\n" | "\r\n") {
                    lexer.extras.line += 1;
                    lexer.extras.col = span.1;
                } else {
                    tokens.push(t.clone());
                }

                match text {
                    "{" | "(" | "[" => pairs.push(t),
                    "}" | ")" | "]" => {
                        pairs.pop_if(|t| t.text() == p[text]).or_else(|| {
                            pairs.push(t);
                            None
                        });
                    }
                    _ => {}
                };
            }
            Err(kind) => GraceError::new(
                &mut errors,
                kind,
                &T::new_token(TK::default(), source, span, line_col),
            ),
        }
    }

    for token in pairs {
        let text = token.text();
        GraceError::unmatched_pair(
            &mut errors,
            &token,
            format!("No `{}` found for `{text}`", p[text]),
        );
    }

    (tokens, errors)
}

fn reserved_keyword<'a, T>(lex: &mut Lexer<'a, T>) -> Result<(), LexingError>
where
    T: Logos<'a, Source = str, Error = LexingError, Extras = LexerExtras>,
{
    let text = lex.slice();
    Err(LexingError::ReservedKeyword(format!(
        "{text} is a reserved keyword that is not used in the language yet",
    )))
}
