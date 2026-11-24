use std::fmt::{Display, Formatter, Result};

use crate::core::errors::kind::GraceErrorKind;

pub mod kind;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraceError {
    pub kind: GraceErrorKind,
    pub line: usize,
    pub col: usize,
}

impl GraceError {
    pub fn new(kind: GraceErrorKind, line: usize, col: usize) -> Self {
        Self { kind, line, col }
    }

    pub fn syntax_error(
        reason: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(GraceErrorKind::SyntaxError(reason.into()), line, col)
    }

    pub fn close_to_expected_text(
        expected: impl Into<String>,
        found: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(
            GraceErrorKind::CloseToExpectedText {
                expected: expected.into(),
                found: found.into(),
            },
            line,
            col,
        )
    }

    pub fn expected_token(
        expected: impl Into<String>,
        found: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(
            GraceErrorKind::ExpectedToken {
                expected: expected.into(),
                found: found.into(),
            },
            line,
            col,
        )
    }

    pub fn expected_text(
        expected: impl Into<String>,
        found: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(
            GraceErrorKind::ExpectedText {
                expected: expected.into(),
                found: found.into(),
            },
            line,
            col,
        )
    }

    pub fn reserved_keyword(
        keyword: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(GraceErrorKind::ReservedKeyword(keyword.into()), line, col)
    }

    pub fn trailing_comma(line: usize, col: usize) -> Self {
        Self::new(GraceErrorKind::TrailingComma, line, col)
    }

    pub fn unexpected_token(
        token: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(GraceErrorKind::UnexpectedToken(token.into()), line, col)
    }

    pub fn unexpected_eof(line: usize, col: usize) -> Self {
        Self::new(GraceErrorKind::UnexpectedEOF, line, col)
    }

    pub fn unmatched_token_pair(
        token: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(GraceErrorKind::UnmatchedTokenPair(token.into()), line, col)
    }

    pub fn custom(reason: impl Into<String>, line: usize, col: usize) -> Self {
        Self::new(GraceErrorKind::Custom(reason.into()), line, col)
    }
}

impl Display for GraceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}:{}] {}", self.line, self.col, self.kind)
    }
}
