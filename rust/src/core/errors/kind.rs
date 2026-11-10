use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraceErrorKind {
    SyntaxError(String),
    CloseToExpectedText(String),
    ExpectedToken(String),
    ExpectedText(String),
    ReservedKeyword(String),
    TrailingComma,
    UnexpectedToken(String),
    UnexpectedEOF,
    UnmatchedTokenPair(String),
    Custom(String),
    #[default]
    Unknown,
}

impl Display for GraceErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{self:?}")
    }
}

impl Error for GraceErrorKind {}
