use anyhow::Result;
use logos::{Lexer, Logos, Skip};

use crate::api::lexers::{GraceError, LexerExtras, LexingError};

pub trait Token<TokenKind> {
    fn nt(
        kind: TokenKind,
        source: &str,
        span: (usize, usize),
        extras: LexerExtras,
    ) -> Self;
}

pub fn extras_newline<'a, T>(lex: &mut Lexer<'a, T>) -> Skip
where
    T: Logos<'a, Source = str, Error = LexingError, Extras = LexerExtras>,
{
    lex.extras.line += 1;
    lex.extras.col = lex.span().end;
    Skip
}

pub fn tokenize<'a, T, TK>(source: &'a str) -> Result<Vec<T>, Vec<GraceError>>
where
    T: Token<TK> + Clone,
    TK: Logos<'a, Source = str, Error = LexingError, Extras = LexerExtras>
        + 'static,
{
    let mut lexer = TK::lexer(source);
    lexer.extras = LexerExtras { line: 1, col: 0 };
    let mut tokens = vec![];
    let mut errors = vec![];
    let mut pairs = vec![];

    while let Some(t) = lexer.next() {
        let span = lexer.span();
        lexer.extras.col = span.start - lexer.extras.col + 1;
        match t {
            Ok(token) => {
                let t =
                    T::nt(token, source, (span.start, span.end), lexer.extras);

                tokens.push(t.clone());
                parse_pair(
                    lexer.extras,
                    &source[span.start..span.end],
                    &mut pairs,
                );
            }
            Err(kind) => {
                errors.push(GraceError {
                    kind,
                    text: source[span.start..span.end].to_string(),
                    reason: String::new(),
                    line: lexer.extras.line,
                    col: lexer.extras.col,
                });
            }
        }
    }

    for (token, text) in pairs {
        match text {
            "{" | "(" | "[" => {
                errors.push(GraceError {
                    kind: LexingError::MissingClosingPair,
                    text: text.to_string(),
                    reason: String::new(),
                    line: token.line,
                    col: token.col,
                });
            }

            "}" | ")" | "]" => {
                errors.push(GraceError {
                    kind: LexingError::UnexpectedToken,
                    text: text.to_string(),
                    reason: String::new(),
                    line: token.line,
                    col: token.col,
                });
            }

            _ => {}
        }
    }

    if errors.is_empty() { Ok(tokens) } else { Err(errors) }
}

fn parse_pair<'a>(
    token: LexerExtras,
    text: &'a str,
    pairs: &mut Vec<(LexerExtras, &'a str)>,
) {
    match text {
        "{" | "(" | "[" => pairs.push((token, text)),

        "}" => {
            pairs.pop_if(|(_, t)| *t == "{").or_else(|| {
                pairs.push((token, text));
                None
            });
        }
        ")" => {
            pairs.pop_if(|(_, t)| *t == "(").or_else(|| {
                pairs.push((token, text));
                None
            });
        }
        "]" => {
            pairs.pop_if(|(_, t)| *t == "[").or_else(|| {
                pairs.push((token, text));
                None
            });
        }

        _ => {}
    };
}
