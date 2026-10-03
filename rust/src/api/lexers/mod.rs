use std::error::Error;
use std::fmt::{Display, Formatter, Result};

pub mod java;
pub mod rust;
pub mod sql;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexerExtras {
    pub line: usize,
    pub col: usize,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    FloatParseError,
    IntegerParseError,
    MissingClosingPair,
    UnexpectedToken,
    #[default]
    Unknown,
}

impl Display for LexingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{self:?}")
    }
}

impl Error for LexingError {}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GraceError {
    pub kind: LexingError,
    pub text: String,
    pub reason: String,
    pub line: usize,
    pub col: usize,
}

impl Display for GraceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "[{}:{}]\t{}: {}",
            self.line, self.col, self.kind, self.reason
        )
    }
}

impl Error for GraceError {}
