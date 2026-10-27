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

#[derive(Default, Debug, Clone, PartialEq)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct Token<'a, TK>
where
    TK: Logos<'a, Source = str, Error = LexingError, Extras = LexerExtras>
        + Default,
{
    pub kind: TK,
    pub text: &'a str,
    pub line: usize,
    pub col: usize,
}

impl<'a, TK> Token<'a, TK>
where
    TK: Logos<'a, Source = str, Error = LexingError, Extras = LexerExtras>
        + Default
        + Clone,
{
    #[flutter_rust_bridge::frb(ignore)]
    pub fn new(kind: TK, text: &'a str, extras: LexerExtras) -> Self {
        Self { kind, text, line: extras.line, col: extras.col }
    }

    #[flutter_rust_bridge::frb(ignore)]
    pub fn tokenize(source: &'a str) -> (Vec<Self>, Vec<GraceError>) {
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

            let mut extras = lexer.extras;
            extras.col = span.0 - extras.col + 1;
            match t {
                Ok(kind) => {
                    if matches!(text, "\n" | "\r\n") {
                        lexer.extras.line += 1;
                        lexer.extras.col = span.1;
                        continue;
                    }

                    let t = Self::new(kind, text, extras);
                    tokens.push(t.clone());
                    match text {
                        "{" | "(" | "[" => pairs.push(t),
                        "}" | ")" | "]" => {
                            pairs.pop_if(|t| t.text == p[text]).or_else(|| {
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
                    &Self::new(TK::default(), text, extras),
                ),
            }
        }

        for token in pairs {
            let text = token.text;
            GraceError::unmatched_pair(
                &mut errors,
                &token,
                format!("No `{}` found for `{text}`", p[text]),
            );
        }

        (tokens, errors)
    }
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
