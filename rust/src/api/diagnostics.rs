use std::ops::{Deref, Range};
use std::slice::Iter;
use std::vec::IntoIter;

use oak_core::{OakError, OakErrorKind};
use unicode_segmentation::UnicodeSegmentation;

use crate::api::grade::Grade;

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
    pub lines: Vec<usize>,
}

impl Diagnostics {
    #[flutter_rust_bridge::frb(ignore)]
    pub fn new(s: &str, e: Vec<OakError>) -> Self {
        let mut d = vec![];
        let lines = Self::line_starts(s);

        for e in e {
            let grade = e.kind().into();
            let (line, col) = e
                .source_offset()
                .map(|o| Self::span_to_linecol(s, &lines, o))
                .unzip();

            d.push(Diagnostic::new(
                grade,
                Self::err_to_str(e.kind()),
                line,
                col,
            ));
        }

        Self { d, lines }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn grade(&self) -> Grade {
        self.iter().map(|d| d.grade).min().unwrap_or(Grade::Easy)
    }

    #[flutter_rust_bridge::frb(ignore)]
    pub fn record_again(&mut self, message: impl Into<String>) {
        self.d.push(Diagnostic::new(Grade::Again, message, None, None));
    }

    #[flutter_rust_bridge::frb(ignore)]
    pub fn record_hard(&mut self, message: impl Into<String>) {
        self.d.push(Diagnostic::new(Grade::Hard, message, None, None));
    }

    #[flutter_rust_bridge::frb(ignore)]
    pub fn record_good(&mut self, message: impl Into<String>) {
        self.d.push(Diagnostic::new(Grade::Good, message, None, None));
    }

    fn err_to_str(err: &OakErrorKind) -> String {
        match err {
            OakErrorKind::SyntaxError { message, .. } => {
                format!("Syntax Error: {message}")
            }
            OakErrorKind::UnexpectedCharacter { character, .. } => {
                format!("Unexpected Character: {character}")
            }
            OakErrorKind::UnexpectedToken { token, .. } => {
                format!("Unexpected Token: {token}")
            }
            OakErrorKind::UnexpectedEof { .. } => format!("Unexpected EOF"),
            OakErrorKind::CustomError { message } => format!("{message}"),
            OakErrorKind::ExpectedToken { expected, .. } => {
                format!("Expected Token: {expected}")
            }
            OakErrorKind::ExpectedName { name_kind, .. } => {
                format!("Expected Kind: {name_kind}")
            }
            OakErrorKind::TrailingCommaNotAllowed { .. } => {
                format!("Trailing Comma is not allowed")
            }
            OakErrorKind::ParseError { message } => {
                format!("Parse Error: {message}")
            }

            // not used
            OakErrorKind::IoError { error, .. } => {
                format!("IO Error: {error:?}")
            }
            OakErrorKind::XmlError { message } => {
                format!("XML Error: {message}")
            }
            OakErrorKind::InvalidTheme { message } => {
                format!("Invalid Theme: {message}")
            }
            OakErrorKind::UnsupportedFormat { format } => {
                format!("Unsupported Format: {format}")
            }
            OakErrorKind::ColorParseError { color } => {
                format!("Color Parse Error: {color}")
            }
            OakErrorKind::FormatError { message } => {
                format!("Format Error: {message}")
            }
            OakErrorKind::SemanticError { message } => {
                format!("Semantic Error: {message}")
            }
            OakErrorKind::ProtocolError { message } => {
                format!("Protocol Error: {message}")
            }
            OakErrorKind::TestFailure { expected, actual, .. } => format!(
                "Test Failure: Expected `{expected}` | Found `{actual}`"
            ),
            OakErrorKind::TestRegenerated { path } => {
                format!("Test Regenerated at: {path:?}")
            }
            OakErrorKind::SerdeError { message } => {
                format!("Serde Error: {message}")
            }
            OakErrorKind::DeserializeError { message } => {
                format!("Deserialize Error: {message}")
            }
            OakErrorKind::ZipError { message } => {
                format!("Zip Error: {message}")
            }
            OakErrorKind::InternalError { message } => {
                format!("Internal Error: {message}")
            }
        }
    }

    fn line_starts(source: &str) -> Vec<usize> {
        let mut s = vec![0];
        for (i, b) in source.bytes().enumerate() {
            if b == b'\n' {
                s.push(i + 1);
            }
        }

        s
    }

    fn span_to_linecol(
        source: &str,
        lines: &Vec<usize>,
        offset: usize,
    ) -> (usize, usize) {
        let start = (0..=offset.min(source.len()))
            .rev()
            .find(|&i| source.is_char_boundary(i))
            .unwrap();

        let idx = lines.partition_point(|&o| o <= start).saturating_sub(1);
        let o = lines[idx];
        let line = lines.iter().filter(|&&a| a <= start).count();
        let col = source[o..start].graphemes(true).count();

        (line, col + 1)
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
