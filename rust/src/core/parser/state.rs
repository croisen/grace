use std::fmt::Debug;

use logos::Logos;

use crate::core::edit::{close_enough, edit_distance_ins, edit_distance_sen};
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

    pub fn get_token(&mut self) -> Option<&Token<'a, TK>> {
        let Some(token) = self.token_current() else {
            self.record_unexpected_eof();
            return None;
        };

        Some(token)
    }

    pub fn get_token_at(&mut self, offset: isize) -> Option<&Token<'a, TK>> {
        let Some(token) = self.token_peek_at(offset) else {
            self.record_unexpected_eof();
            return None;
        };

        Some(token)
    }

    pub fn token_current(&self) -> Option<&Token<'a, TK>> {
        self.tokens.get(self.index)
    }

    pub fn token_text(&mut self) -> Option<&'a str> {
        self.tokens.get(self.index).map(|t| t.text)
    }

    pub fn token_kind(&mut self) -> Option<&TK> {
        self.tokens.get(self.index).map(|t| &t.kind)
    }

    pub fn token_peek_at(&self, offset: isize) -> Option<&Token<'a, TK>> {
        self.tokens.get(self.index.wrapping_add_signed(offset))
    }

    pub fn token_peek_at_text(&mut self, offset: isize) -> Option<&'a str> {
        self.token_peek_at(offset).map(|t| t.text)
    }

    pub fn token_peek_at_kind(&mut self, offset: isize) -> Option<&TK> {
        self.token_peek_at(offset).map(|t| &t.kind)
    }

    pub fn token_eat_kind(&mut self, kind: TK) -> bool {
        let Some(token) = self.get_token() else { return false };
        token.kind == kind
    }

    pub fn token_eat_kinds(&mut self, kind: &[TK]) -> bool {
        let Some(token) = self.get_token() else { return false };
        for kind in kind.iter() {
            if &token.kind == kind {
                return true;
            }
        }

        false
    }

    /// case _ins_ensitive
    pub fn token_eat_text_ins(&mut self, text: impl AsRef<str>) -> bool {
        let Some(token) = self.get_token() else { return false };
        let distance = edit_distance_ins(text.as_ref(), token.text); // 0 is actual match
        let output = close_enough(text.as_ref(), distance);
        if distance != 0 && output {
            self.record_close_enough(text.as_ref());
        }

        output
    }

    /// case _ins_ensitive
    pub fn token_eat_texts_ins(&mut self, text: &[impl AsRef<str>]) -> bool {
        let Some(_) = self.get_token() else { return false };
        for text in text.iter() {
            if self.token_eat_text_ins(text) {
                return true;
            }
        }

        false
    }

    /// case _sen_sitive
    pub fn token_eat_text_sen(&mut self, text: impl AsRef<str>) -> bool {
        let Some(token) = self.get_token() else { return false };
        let distance = edit_distance_sen(text.as_ref(), token.text); // 0 is actual match
        let output = close_enough(text.as_ref(), distance);
        if distance != 0 && output {
            self.record_close_enough(text.as_ref());
        }

        output
    }

    /// case _sen_sitive
    pub fn token_eat_texts_sen(&mut self, text: &[impl AsRef<str>]) -> bool {
        let Some(_) = self.get_token() else { return false };
        for text in text.iter() {
            if self.token_eat_text_sen(text) {
                return true;
            }
        }

        false
    }

    pub fn token_expect_kind(&mut self, kind: TK) -> bool {
        let res = self.token_eat_kind(kind.clone());
        if !res {
            self.record_expected_token(format!("{kind:?}"));
        }

        res
    }

    pub fn token_expect_kinds(&mut self, kind: &[TK]) -> bool {
        for kind in kind.iter() {
            if self.token_eat_kind(kind.clone()) {
                return true;
            }
        }

        self.record_expected_token(format!("{kind:?}"));
        false
    }

    /// case _ins_ensitive
    pub fn token_expect_text_ins(&mut self, text: impl AsRef<str>) -> bool {
        let Some(token) = self.get_token() else { return false };
        let distance = edit_distance_ins(text.as_ref(), token.text); // 0 is actual match
        let output = close_enough(text.as_ref(), distance);
        if distance != 0 && output {
            self.record_close_enough(text.as_ref());
        } else if distance != 0 {
            self.record_expected_text(text.as_ref().to_uppercase());
        }

        output
    }

    /// case _sen_sitive
    pub fn token_expect_text_sen(&mut self, text: impl AsRef<str>) -> bool {
        let Some(token) = self.get_token() else { return false };
        let distance = edit_distance_sen(text.as_ref(), token.text); // 0 is actual match
        let output = close_enough(text.as_ref(), distance);
        if distance != 0 && output {
            self.record_close_enough(text.as_ref());
        } else if distance != 0 {
            self.record_expected_text(text.as_ref().to_uppercase());
        }

        output
    }

    /// Match and consume a token. Unlike `token_eat_*`, this advances on success.
    pub fn token_take_text_sen(&mut self, text: impl AsRef<str>) -> bool {
        if self.token_eat_text_sen(text) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn token_expect_take_text_sen(
        &mut self,
        text: impl AsRef<str>,
    ) -> bool {
        if self.token_expect_text_sen(text) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn token_take_kind(&mut self, kind: TK) -> bool {
        if self.token_eat_kind(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn token_take_kinds(&mut self, kinds: &[TK]) -> bool {
        if self.token_eat_kinds(kinds) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn token_take_text_ins(&mut self, text: impl AsRef<str>) -> bool {
        if self.token_eat_text_ins(text) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn token_take_texts_ins(&mut self, texts: &[impl AsRef<str>]) -> bool {
        if self.token_eat_texts_ins(texts) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn token_expect_take_text_ins(
        &mut self,
        text: impl AsRef<str>,
    ) -> bool {
        if self.token_expect_text_ins(text) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn record_syntax_error(&mut self, reason: impl Into<String>) {
        let Some(token) = &self.token_current() else { return };
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::syntax_error(reason, line, col));
    }

    pub fn record_close_enough(&mut self, text: impl Into<String>) {
        let Some(token) = &self.token_current() else { return };
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::close_to_expected_text(
            text, token.text, line, col,
        ));
    }

    pub fn record_expected_token(&mut self, text: impl Into<String>) {
        let Some(token) = &self.token_current() else { return };
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::expected_token(
            text,
            format!("{:?}", token.kind),
            line,
            col,
        ));
    }

    pub fn record_expected_text(&mut self, text: impl Into<String>) {
        let Some(token) = &self.token_current() else { return };
        let (line, col) = (token.line, token.col);
        self.errors
            .push(GraceError::expected_text(text, token.text, line, col));
    }

    // not really used since the lexer takes care of it
    pub fn record_reserved_keyword(&mut self, reason: impl Into<String>) {
        let Some(token) = &self.token_current() else { return };
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::reserved_keyword(reason, line, col));
    }

    pub fn record_trailing_comma(&mut self) {
        let Some(token) = &self.token_current() else { return };
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::trailing_comma(line, col));
    }

    pub fn record_unexpected_token(&mut self) {
        let Some(token) = &self.token_current() else { return };
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::unexpected_token(token.text, line, col));
    }

    pub fn record_unexpected_eof(&mut self) {
        let t = Token::new(TK::default(), "", LexerExtras { line: 1, col: 0 });
        let token = self.tokens.last().unwrap_or(&t);
        let (line, col) = (token.line, token.col + token.text.len());
        self.errors.push(GraceError::unexpected_eof(line, col));
    }

    pub fn record_unmatched_token_pair(&mut self, reason: impl Into<String>) {
        let Some(token) = &self.token_current() else { return };
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::unmatched_token_pair(reason, line, col));
    }

    pub fn record_custom(&mut self, reason: impl Into<String>) {
        let Some(token) = &self.token_current() else { return };
        let (line, col) = (token.line, token.col);
        self.errors.push(GraceError::custom(reason, line, col));
    }

    pub fn seek(&mut self, idx: usize) {
        self.index = idx;
    }

    pub fn seek_end(&mut self) {
        self.seek(self.tokens.len().wrapping_sub(1))
    }

    pub fn seek_offset(&mut self, offset: isize) {
        self.seek(self.index.wrapping_add_signed(offset))
    }
}
