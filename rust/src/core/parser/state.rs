use std::fmt::Debug;

use logos::Logos;

use crate::core::errors::GraceError;
use crate::core::errors::kind::GraceErrorKind;
use crate::core::lexers::{LexerExtras, Token};

#[flutter_rust_bridge::frb(ignore)]
pub struct ParserState<'a, TK, L>
where
    TK: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Debug,
{
    pub tokens: Vec<Token<'a, TK>>,
    pub index: usize,
    pub root: Vec<L>,
    pub errors: Vec<GraceError>,
}

impl<'a, TK, L> ParserState<'a, TK, L>
where
    TK: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Debug,
{
    pub fn new(source: &'a str) -> Self {
        let (tokens, errors) = Token::tokenize(source);
        Self { tokens, index: 0, errors, root: vec![] }
    }

    pub fn ast(&self) -> &Vec<L> {
        &self.root
    }

    pub fn errors(&self) -> &Vec<GraceError> {
        &self.errors
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn not_at_end(&self) -> bool {
        self.index < self.tokens.len()
    }

    pub fn token_current(&self) -> Option<&Token<'a, TK>> {
        self.tokens.get(self.index)
    }

    pub fn token_peek_at(&self, offset: isize) -> Option<&Token<'a, TK>> {
        self.tokens.get(self.index.wrapping_add_signed(offset))
    }

    pub fn token_eat(&mut self, kind: TK) -> bool {
        if let Some(token) = self.token_current() {
            return token.kind == kind;
        }

        return false;
    }

    pub fn token_expect(&mut self, kind: TK) -> Result<(), GraceError> {
        if self.token_eat(kind) {
            return Ok(());
        }

        if self.token_current().is_none() {
            let err = self.record_unexpected_eof();
            return Err(err);
        }

        let err = self.record_expected_token(format!("{kind:?}"));
        Err(err)
    }

    pub fn record_syntax_error(
        &mut self,
        reason: impl Into<String>,
    ) -> GraceError {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        let err = GraceError::syntax_error(reason, line, col);
        self.errors.push(err.clone());
        err
    }

    pub fn record_expected_token(
        &mut self,
        reason: impl Into<String>,
    ) -> GraceError {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        let err = GraceError::expected_token(reason, line, col);
        self.errors.push(err.clone());
        err
    }

    pub fn record_reserved_keyword(
        &mut self,
        reason: impl Into<String>,
    ) -> GraceError {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        let err = GraceError::reserved_keyword(reason, line, col);
        self.errors.push(err.clone());
        err
    }

    pub fn record_unexpected_token(
        &mut self,
        reason: impl Into<String>,
    ) -> GraceError {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        let err = GraceError::syntax_error(reason, line, col);
        self.errors.push(err.clone());
        err
    }

    pub fn record_unexpected_eof(&mut self) -> GraceError {
        self.seek_end();
        let token = &self.tokens[self.tokens.len() - 1];
        let (line, col) = (token.line, token.col + 1);
        let err = GraceError::unexpected_eof(line, col);
        self.errors.push(err.clone());
        err
    }

    pub fn record_unmatched_token_pair(
        &mut self,
        reason: impl Into<String>,
    ) -> GraceError {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        let err = GraceError::unmatched_token_pair(reason, line, col);
        self.errors.push(err);
        err
    }

    pub fn record_custom(&mut self, reason: impl Into<String>) -> GraceError {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        let err = GraceError::custom(reason, line, col);
        self.errors.push(err);
        err
    }

    fn seek(&mut self, idx: usize) {
        self.index = idx;
    }

    fn seek_end(&mut self) {
        self.seek(self.index - 1)
    }
}
