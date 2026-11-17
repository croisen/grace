use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};
use std::num::{ParseFloatError, ParseIntError};

use logos::{Lexer, Logos};

use crate::core::errors::GraceError;
use crate::core::errors::kind::GraceErrorKind;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexerExtras {
    pub line: usize,
    pub col: usize,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Token<'a, TK>
where
    TK: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Clone
        + PartialEq
        + Debug,
{
    pub kind: TK,
    pub text: &'a str,
    pub line: usize,
    pub col: usize,
}

impl<'a, TK> Token<'a, TK>
where
    TK: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Clone
        + PartialEq
        + Debug,
{
    pub fn new(kind: TK, text: &'a str, extras: LexerExtras) -> Self {
        Self { kind, text, line: extras.line, col: extras.col }
    }

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
                Err(kind) => {
                    errors.push(GraceError::new(kind, extras.line, extras.col))
                }
            }
        }

        for token in pairs {
            let (text, line, col) = (token.text, token.line, token.col);
            errors.push(GraceError::unmatched_token_pair(text, line, col));
        }

        (tokens, errors)
    }
}

pub fn reserved_keyword<'a, T>(
    lex: &mut Lexer<'a, T>,
) -> Result<(), GraceErrorKind>
where
    T: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>,
{
    let text = lex.slice();
    Err(GraceErrorKind::ReservedKeyword(format!(
        "{text} is a reserved keyword that is not used in the language yet",
    )))
}
