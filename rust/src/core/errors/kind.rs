use std::fmt::{Debug, Display, Formatter, Result as FResult};

#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub enum GraceErrorKind {
    SyntaxError(String),
    ExpectedToken(String),
    ReservedKeyword(String),
    UnexpectedToken(String),
    UnexpectedEOF,
    UnmatchedTokenPair(String),
    Custom(String),
    #[default]
    Unknown(String),
}

impl Display for GraceErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{self:?}")
    }
}

impl Error for GraceErrorKind {}
