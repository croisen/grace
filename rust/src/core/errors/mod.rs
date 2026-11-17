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
        reason: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(GraceErrorKind::CloseToExpectedText(reason.into()), line, col)
    }

    pub fn expected_token(
        reason: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(GraceErrorKind::ExpectedToken(reason.into()), line, col)
    }

    pub fn expected_text(
        reason: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(GraceErrorKind::ExpectedText(reason.into()), line, col)
    }

    pub fn reserved_keyword(
        reason: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(GraceErrorKind::ReservedKeyword(reason.into()), line, col)
    }

    pub fn trailing_comma(line: usize, col: usize) -> Self {
        Self::new(GraceErrorKind::TrailingComma, line, col)
    }

    pub fn unexpected_token(
        reason: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(GraceErrorKind::UnexpectedToken(reason.into()), line, col)
    }

    pub fn unexpected_eof(line: usize, col: usize) -> Self {
        Self::new(GraceErrorKind::UnexpectedEOF, line, col)
    }

    pub fn unmatched_token_pair(
        reason: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self::new(GraceErrorKind::UnmatchedTokenPair(reason.into()), line, col)
    }

    pub fn custom(reason: impl Into<String>, line: usize, col: usize) -> Self {
        Self::new(GraceErrorKind::Custom(reason.into()), line, col)
    }
}
