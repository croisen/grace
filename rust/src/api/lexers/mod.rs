use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use logos::Logos;

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
    FloatParseError,
    IntegerParseError,
    SyntaxError,
    UnmatchedPair,
    UnexpectedToken,
    #[default]
    Unknown,
}

impl Display for LexingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{self:?}")
    }
}

impl Error for LexingError {}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GraceError {
    pub kind: LexingError,
    pub text: String,
    pub reason: String,
    pub line: usize,
    pub col: usize,
}

impl GraceError {
    pub fn new(
        kind: LexingError,
        text: String,
        reason: String,
        line: usize,
        col: usize,
    ) -> Self {
        Self { kind, text, reason, line, col }
    }
}

impl Display for GraceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "[{}:{}]\t\t{}: {}",
            self.line, self.col, self.kind, self.reason
        )
    }
}

impl Error for GraceError {}

trait Token<'a> {
    #[flutter_rust_bridge::frb(ignore)]
    type TK;
    #[flutter_rust_bridge::frb(ignore)]
    fn new_token(
        kind: Self::TK,
        source: &'a str,
        span: (usize, usize),
        line_col: (usize, usize),
    ) -> Self;
}

#[flutter_rust_bridge::frb(ignore)]
fn tokenize<'a, T, TK>(source: &'a str) -> (Vec<T>, Vec<GraceError>)
where
    T: Token<'a, TK = TK> + Clone,
    TK: Logos<'a, Source = str, Error = LexingError, Extras = LexerExtras>,
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
        let text = &source[span.0..span.1];
        let LexerExtras { line, mut col } = lexer.extras;
        col = span.0 - col + 1;
        match t {
            Ok(token) => {
                let t = T::new_token(token, source, span, (line, col));
                tokens.push(t.clone());
                match text {
                    "{" | "(" | "[" => pairs.push((line, col, text)),
                    "}" | ")" | "]" => {
                        pairs.pop_if(|(_, _, t)| *t == p[text]).or_else(|| {
                            pairs.push((line, col, text));
                            None
                        });
                    }
                    _ => {}
                };
            }
            Err(kind) => {
                let err =
                    GraceError::new(kind, text.into(), "".into(), line, col);
                errors.push(err);
            }
        }

        // let's just do this manually???
        if text == "\n" || text == "\r\n" {
            lexer.extras.line += 1;
            lexer.extras.col = span.1;
        }
    }

    for (line, col, text) in pairs {
        match text {
            "{" | "(" | "[" => {
                errors.push(GraceError::new(
                    LexingError::UnmatchedPair,
                    text.into(),
                    format!("No `{}` found for `{}`", p[text], text),
                    line,
                    col,
                ));
            }

            "}" | ")" | "]" => {
                errors.push(GraceError::new(
                    LexingError::UnmatchedPair,
                    text.into(),
                    format!("No `{}` found for `{}`", p[text], text),
                    line,
                    col,
                ));
            }

            _ => {}
        }
    }

    (tokens, errors)
}
