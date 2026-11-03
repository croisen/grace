use std::fmt::Debug;
use std::marker::PhantomData;

use logos::Logos;

use crate::core::errors::GraceError;
use crate::core::errors::kind::GraceErrorKind;
use crate::core::lexers::{LexerExtras, Token};

pub struct ParserState<'a, TK, L>
where
    TK: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Clone
        + PartialEq
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
        + Clone
        + PartialEq
        + Debug,
{
    pub fn new(source: &'a str) -> Self {
        let (tokens, errors) = Token::tokenize(source);
        Self { tokens, index: 0, errors, root: vec![] }
    }

    pub fn root_push(&mut self, item: impl Into<L>) {
        self.root.push(item.into())
    }

    pub fn errors(&self) -> &Vec<GraceError> {
        &self.errors
    }

    pub fn advance(&mut self) {
        self.seek_offset(1);
    }

    pub fn not_at_end(&self) -> bool {
        self.index < self.tokens.len()
    }

    pub fn token_current(&self) -> Option<&Token<'a, TK>> {
        self.tokens.get(self.index)
    }

    pub fn token_peek_text(&mut self) -> Option<&'a str> {
        Some(self.get_token().ok()?.text)
    }

    pub fn token_peek_at(&self, offset: isize) -> Option<&Token<'a, TK>> {
        self.tokens.get(self.index.wrapping_add_signed(offset))
    }

    pub fn token_peek_at_text(&mut self, offset: isize) -> Option<&'a str> {
        Some(self.get_token_at(offset).ok()?.text)
    }

    pub fn token_eat(&mut self) -> bool {
        let token = self.get_token();
        token.is_ok()
    }

    pub fn token_eat_kind(&mut self, kind: TK) -> bool {
        let token = self.get_token();
        if token.is_err() {
            return false;
        }

        token.unwrap().kind == kind
    }

    /// case _ins_ensitive
    pub fn token_eat_text_ins(&mut self, text: impl AsRef<str>) -> bool {
        let token = self.get_token();
        if token.is_err() {
            return false;
        }

        token.unwrap().text.eq_ignore_ascii_case(text.as_ref())
    }

    /// case _ins_ensitive
    pub fn token_eat_texts_ins(&mut self, text: &[impl AsRef<str>]) -> bool {
        let token = self.get_token();
        if token.is_err() {
            return false;
        }

        let token = token.unwrap();
        for text in text.iter() {
            if token.text.eq_ignore_ascii_case(text.as_ref()) {
                return true;
            }
        }

        false
    }

    /// case _sen_sitive
    pub fn token_eat_text_sen(&mut self, text: impl AsRef<str>) -> bool {
        let token = self.get_token();
        if token.is_err() {
            return false;
        }

        token.unwrap().text == text.as_ref()
    }

    /// case _sen_sitive
    pub fn token_eat_texts_sen(&mut self, text: &[impl AsRef<str>]) -> bool {
        let token = self.get_token();
        if token.is_err() {
            return false;
        }

        let token = token.unwrap();
        for text in text.iter() {
            if token.text == text.as_ref() {
                return true;
            }
        }

        false
    }

    pub fn token_expect_kind(&mut self, kind: TK) -> bool {
        let token = self.get_token();
        if token.is_err() {
            return false;
        }

        let res = token.unwrap().kind == kind;
        if !res {
            self.record_expected_token(format!("{kind:?}"));
        }

        res
    }

    pub fn token_expect_kinds(&mut self, kind: &[TK]) -> bool {
        let token = self.get_token();
        if token.is_err() {
            return false;
        }

        let token = token.unwrap();
        for kind in kind.iter() {
            if &token.kind == kind {
                return true;
            }
        }

        self.record_expected_token(format!("{kind:?}"));
        false
    }

    /// case _ins_ensitive
    pub fn token_expect_text_ins(&mut self, text: impl AsRef<str>) -> bool {
        let token = self.get_token();
        if token.is_err() {
            return false;
        }

        let res = token.unwrap().text.eq_ignore_ascii_case(text.as_ref());
        if !res {
            self.record_expected_text(text.as_ref().to_uppercase());
        }

        res
    }

    /// case _sen_sitive
    pub fn token_expect_text_sen(&mut self, text: impl AsRef<str>) -> bool {
        let token = self.get_token();
        if token.is_err() {
            return false;
        }

        let res = token.unwrap().text == text.as_ref();
        if !res {
            self.record_expected_text(text.as_ref());
        }

        res
    }

    pub fn record_syntax_error(&mut self, reason: impl Into<String>) {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::syntax_error(reason, line, col));
    }

    pub fn record_expected_token(&mut self, text: impl Into<String>) {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        let reason = format!("`{}` found `{:?}`", text.into(), token.kind);
        self.errors.push(GraceError::expected_token(reason, line, col));
    }

    pub fn record_expected_text(&mut self, text: impl Into<String>) {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        let reason = format!("`{}` found `{}`", text.into(), token.text);
        self.errors.push(GraceError::expected_text(reason, line, col));
    }

    pub fn record_reserved_keyword(&mut self, reason: impl Into<String>) {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::reserved_keyword(reason, line, col));
    }

    pub fn record_trailing_comma(&mut self) {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::trailing_comma(line, col));
    }

    pub fn record_unexpected_token(&mut self) {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::unexpected_token(token.text, line, col));
    }

    pub fn record_unexpected_eof(&mut self) -> GraceError {
        if !self.not_at_end() {
            self.seek_end();
        }

        let t = Token::new(TK::default(), "", LexerExtras { line: 1, col: 0 });
        let token = self.tokens.get(self.index).unwrap_or(&t);
        let (line, col) = (token.line, token.col + token.text.len());
        let err = GraceError::unexpected_eof(line, col);
        self.errors.push(err.clone());
        err
    }

    pub fn record_unmatched_token_pair(&mut self, reason: impl Into<String>) {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::unmatched_token_pair(reason, line, col));
    }

    pub fn record_custom(&mut self, reason: impl Into<String>) {
        let token = &self.tokens[self.index];
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::custom(reason, line, col));
    }

    pub fn seek(&mut self, idx: usize) {
        self.index = idx;
    }

    pub fn seek_end(&mut self) {
        self.seek(self.index - 1)
    }

    pub fn seek_offset(&mut self, offset: isize) {
        self.seek(self.index.wrapping_add_signed(offset))
    }

    pub fn get_token(&mut self) -> Result<&Token<'a, TK>, GraceError> {
        if let Some(token) = self.tokens.get(self.index) {
            Ok(token)
        } else {
            Err(self.record_unexpected_eof())
        }
    }

    pub fn get_token_at(
        &mut self,
        offset: isize,
    ) -> Result<&Token<'a, TK>, GraceError> {
        if let Some(token) =
            self.tokens.get(self.index.wrapping_add_signed(offset))
        {
            Ok(token)
        } else {
            Err(self.record_unexpected_eof())
        }
    }
}
