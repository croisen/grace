use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraceErrorKind {
    SyntaxError(String),
    CloseToExpectedText {
        expected: String,
        found: String,
    },
    ExpectedToken {
        expected: String,
        found: String,
    },
    ExpectedText {
        expected: String,
        found: String,
    },
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
        match self {
            GraceErrorKind::SyntaxError(m) => write!(f, "Syntax Error {m}"),
            Self::CloseToExpectedText { expected, found } => {
                write!(f, "Expected token `{expected}` is close to `{found}`")
            }
            Self::ExpectedToken { expected, found } => {
                write!(f, "Expected token kind `{expected}` found `{found}`")
            }
            Self::ExpectedText { expected, found } => {
                write!(f, "Expected token `{expected}` found `{found}`")
            }
            Self::ReservedKeyword(k) => write!(f, "Reserved keyword `{k}`"),
            Self::TrailingComma => write!(f, "Trailing comma not allowed"),
            Self::UnexpectedToken(t) => write!(f, "Unexpected token `{t}`"),
            Self::UnexpectedEOF => write!(f, "Unexpected EOF"),
            Self::UnmatchedTokenPair(s) => write!(f, "Unmatched `{s}`"),
            Self::Custom(s) => write!(f, "{s}"),
            Self::Unknown => write!(f, "Unknown Error"),
        }
    }
}

impl Error for GraceErrorKind {}
