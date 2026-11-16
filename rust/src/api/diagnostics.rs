use std::ops::Deref;
use std::slice::Iter;
use std::vec::IntoIter;

use oak_core::OakError;
use unicode_segmentation::UnicodeSegmentation;

use crate::api::grade::Grade;

pub struct Diagnostic {
    pub message: String,
    pub grade: Grade,
    pub line: Option<usize>,
    pub col: Option<usize>,
}

impl Diagnostic {
    #[flutter_rust_bridge::frb(sync)]
    pub fn to_string(&self) -> String {
        let (Some(line), Some(col)) = (self.line, self.col) else {
            return format!("[] {}", self.message);
        };

        format!("[{}:{}] {}", line, col, self.message)
    }
}

pub struct Diagnostics {
    pub d: Vec<Diagnostic>,
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

            d.push(Diagnostic { message: e.to_string(), grade, line, col });
        }

        Self { d }
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
