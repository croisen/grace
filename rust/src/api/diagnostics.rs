use std::ops::Deref;
use std::slice::Iter;
use std::vec::IntoIter;

use crate::api::grade::Grade;
use crate::core::errors::GraceError;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub message: String,
    pub grade: Grade,
    pub line: Option<usize>,
    pub col: Option<usize>,
}

impl Diagnostic {
    #[flutter_rust_bridge::frb(ignore)]
    pub fn new(
        grade: Grade,
        message: impl Into<String>,
        line: Option<usize>,
        col: Option<usize>,
    ) -> Self {
        Self { message: message.into(), grade, line, col }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn to_string(&self) -> String {
        let (Some(line), Some(col)) = (self.line, self.col) else {
            return format!("[....] {}", self.message);
        };

        format!("[{}:{}] {}", line, col, self.message)
    }
}

#[flutter_rust_bridge::frb(opaque)]
pub struct Diagnostics {
    pub d: Vec<Diagnostic>,
}

impl Diagnostics {
    #[flutter_rust_bridge::frb(ignore)]
    pub fn new(e: Vec<GraceError>) -> Self {
        Self {
            d: e.into_iter()
                .map(|e| {
                    Diagnostic::new(
                        (&e.kind).into(),
                        e.kind.to_string(),
                        Some(e.line),
                        Some(e.col),
                    )
                })
                .collect(),
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn grade(&self) -> Grade {
        self.iter().map(|d| d.grade).min().unwrap_or(Grade::Easy)
    }

    #[flutter_rust_bridge::frb(ignore)]
    pub fn record_again(
        &mut self,
        message: impl Into<String>,
        line: Option<usize>,
        col: Option<usize>,
    ) {
        self.d.push(Diagnostic::new(Grade::Again, message, line, col));
    }

    #[flutter_rust_bridge::frb(ignore)]
    pub fn record_hard(
        &mut self,
        message: impl Into<String>,
        line: Option<usize>,
        col: Option<usize>,
    ) {
        self.d.push(Diagnostic::new(Grade::Hard, message, line, col));
    }

    #[flutter_rust_bridge::frb(ignore)]
    pub fn record_good(
        &mut self,
        message: impl Into<String>,
        line: Option<usize>,
        col: Option<usize>,
    ) {
        self.d.push(Diagnostic::new(Grade::Good, message, line, col));
    }
}

impl Deref for Diagnostics {
    type Target = Vec<Diagnostic>;

    fn deref(&self) -> &Self::Target {
        &self.d
    }
}

impl IntoIterator for Diagnostics {
    type IntoIter = IntoIter<Diagnostic>;
    type Item = Diagnostic;

    fn into_iter(self) -> Self::IntoIter {
        self.d.into_iter()
    }
}

impl<'a> IntoIterator for &'a Diagnostics {
    type IntoIter = Iter<'a, Diagnostic>;
    type Item = &'a Diagnostic;

    fn into_iter(self) -> Self::IntoIter {
        self.d.iter()
    }
}
