use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use crate::api::lexers::{LexingError, Token};
use crate::utils::macros::all;

pub mod ast;
pub mod drills;
pub mod lexers;
pub mod simple;

#[derive(Default, Debug, Clone)]
pub struct GraceError {
    pub kind: LexingError,
    pub text: String,
    pub line: usize,
    pub col: usize,
}

impl GraceError {
    #[flutter_rust_bridge::frb(ignore)]
    pub fn new<'a, T: Token<'a>>(
        errors: &mut Vec<Self>,
        kind: LexingError,
        token: &T,
    ) {
        errors.push(Self {
            kind,
            text: token.text().into(),
            line: token.line(),
            col: token.col(),
        })
    }

    #[flutter_rust_bridge::frb(ignore)]
    pub fn unmatched_pair<'a, T: Token<'a>>(
        errors: &mut Vec<Self>,
        token: &T,
        reason: impl AsRef<str>,
    ) {
        Self::new(
            errors,
            LexingError::UnmatchedPair(reason.as_ref().into()),
            token,
        )
    }

    #[flutter_rust_bridge::frb(ignore)]
    pub fn syntax_error<'a, T: Token<'a>>(
        errors: &mut Vec<Self>,
        token: &T,
        reason: impl AsRef<str>,
    ) {
        Self::new(
            errors,
            LexingError::SyntaxError(reason.as_ref().into()),
            token,
        )
    }
}

impl Display for GraceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}:{}]\t\t", self.line, self.col)?;
        match &self.kind {
            LexingError::FloatParseError(s) => {
                write!(f, "Float Parse Error: {s}")
            }
            LexingError::IntegerParseError(s) => {
                write!(f, "Integer Parse Error: {s}")
            }
            LexingError::ReservedKeyword(s) => {
                write!(f, "Reserved Keyword Error: {s}")
            }
            LexingError::SyntaxError(s) => {
                write!(f, "Syntax Error: {s}")
            }
            LexingError::UnmatchedPair(s) => {
                write!(f, "Unmatched Pair: {s}")
            }
            LexingError::UnexpectedToken(s) => {
                write!(f, "Unexpected Token: {s}")
            }
            LexingError::Unknown => {
                write!(f, "Unknown Error: {}", self.text)
            }
        }
    }
}

impl Error for GraceError {}

impl PartialEq for GraceError {
    fn eq(&self, other: &Self) -> bool {
        all!(self.line == other.line, self.col != other.col)
    }
}

impl Eq for GraceError {}

impl PartialOrd for GraceError {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(c) = self.line.partial_cmp(&other.line)
            && c != Ordering::Equal
        {
            return Some(c);
        }

        if let Some(c) = self.col.partial_cmp(&other.col)
            && c != Ordering::Equal
        {
            return Some(c);
        }

        Some(Ordering::Equal)
    }
}

impl Ord for GraceError {
    fn cmp(&self, other: &Self) -> Ordering {
        if let c = self.line.cmp(&other.line)
            && c != Ordering::Equal
        {
            return c;
        }

        if let c = self.col.cmp(&other.col)
            && c != Ordering::Equal
        {
            return c;
        }

        Ordering::Equal
    }
}
