use std::fmt::{Display, Formatter, Result};
use std::ops::Sub;

use crate::core::errors::kind::GraceErrorKind;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Grade {
    Again = 1, // will not compile (mistake is easily known)
    Hard = 2,  // will not compile (mistake is not easily known)
    Good = 3,  // close enough (can still be parsed without much error)
    Easy = 4,  // no mistakes
}

impl Grade {
    #[flutter_rust_bridge::frb(sync)]
    pub fn to_string(&self) -> String {
        format!("{self}")
    }
}

impl Sub<usize> for Grade {
    type Output = isize;

    fn sub(self, rhs: usize) -> Self::Output {
        (self as isize).wrapping_sub_unsigned(rhs)
    }
}

impl From<&GraceErrorKind> for Grade {
    fn from(value: &GraceErrorKind) -> Self {
        match value {
            GraceErrorKind::SyntaxError { .. } => Self::Again,
            GraceErrorKind::CloseToExpectedText { .. } => Self::Good,
            GraceErrorKind::ExpectedToken { .. } => Self::Good,
            GraceErrorKind::ExpectedText { .. } => Self::Good,
            GraceErrorKind::ReservedKeyword(_) => Self::Hard,
            GraceErrorKind::TrailingComma => Self::Good,
            GraceErrorKind::UnexpectedToken { .. } => Self::Hard,
            GraceErrorKind::UnexpectedEOF => Self::Hard,
            GraceErrorKind::UnmatchedTokenPair(_) => Self::Hard,
            GraceErrorKind::Custom(_) => Self::Again,
            GraceErrorKind::Unknown => Self::Again,
        }
    }
}

impl Display for Grade {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{self:?}")
    }
}
