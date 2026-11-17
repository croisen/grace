use std::ops::Sub;

use crate::core::errors::GraceError;
use crate::core::errors::kind::GraceErrorKind;

#[derive(Debug, Copy, Clone)]
pub enum Grade {
    Again = 1, // will not compile (mistake is easily known)
    Hard = 2,  // will not compile (mistake is not easily known)
    Good = 3,  // close enough (can still be parsed without much error)
    Easy = 4,  // no mistakes
}

impl Sub<usize> for Grade {
    type Output = isize;

    fn sub(self, rhs: usize) -> Self::Output {
        (self as isize).wrapping_sub_unsigned(rhs)
    }
}

impl From<GraceError> for Grade {
    #[flutter_rust_bridge::frb(sync)]
    fn from(value: GraceError) -> Self {
        match value.kind {
            GraceErrorKind::SyntaxError(_) => Self::Again,
            GraceErrorKind::CloseToExpectedText(_) => Self::Good,
            GraceErrorKind::ExpectedToken(_) => Self::Again,
            GraceErrorKind::ExpectedText(_) => Self::Again,
            GraceErrorKind::ReservedKeyword(_) => Self::Again,
            GraceErrorKind::TrailingComma => Self::Good,
            GraceErrorKind::UnexpectedToken(_) => Self::Hard,
            GraceErrorKind::UnexpectedEOF => Self::Again,
            GraceErrorKind::UnmatchedTokenPair(_) => Self::Hard,
            GraceErrorKind::Custom(_) => Self::Hard,
            GraceErrorKind::Unknown => Self::Hard,
        }
    }
}
