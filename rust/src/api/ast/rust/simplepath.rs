use crate::api::GraceError;
use crate::api::ast::rust::unexpected_eof;
use crate::api::lexers::LexingError;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone, PartialEq)]
#[flutter_rust_bridge::frb(ignore)]
pub struct SimplePath(pub Vec<SimplePathSegment>);

#[derive(Debug, Clone, PartialEq)]
#[flutter_rust_bridge::frb(ignore)]
pub enum SimplePathSegment {
    Identifier(String),
    Super,
    Self1,
    Crate,
    DCrate,
}

impl SimplePath {
    #[flutter_rust_bridge::frb(ignore)]
    pub fn new(
        iter: &mut PrevIter<'_, RustToken>,
        errors: &mut Vec<GraceError>,
    ) -> Self {
        let mut v = vec![];
        let mut found = false;
        while let Some(token) = iter.next() {
            match token.kind {
                RustTokenKind::MemberAccess => {
                    if !found && !v.is_empty() {
                        GraceError::syntax_error(
                            errors,
                            token,
                            "expecting identifier",
                        );
                    }

                    if token.text != "::" {
                        GraceError::syntax_error(
                            errors,
                            token,
                            "expecting `;`",
                        );
                    }

                    found = false;
                }
                RustTokenKind::Identifier | RustTokenKind::StrictKeyword => {
                    if !v.is_empty()
                        && token.kind == RustTokenKind::StrictKeyword
                    {
                        GraceError::syntax_error(
                            errors,
                            token,
                            "expecting identifier",
                        );

                        continue;
                    }

                    if found {
                        GraceError::syntax_error(
                            errors,
                            token,
                            "expecting `;`",
                        );

                        continue;
                    }

                    if let Some(p) = SimplePathSegment::new(iter, errors) {
                        v.push(p);
                        found = true;
                    }
                }
                RustTokenKind::Pairs | RustTokenKind::Delimiters => {
                    let _ = iter.prev();
                    break;
                }
                _ => break,
            }
        }

        Self(v)
    }
}

impl SimplePathSegment {
    pub fn new(
        iter: &mut PrevIter<'_, RustToken>,
        errors: &mut Vec<GraceError>,
    ) -> Option<Self> {
        if let Some(token) = iter.current() {
            Some(match token.text.as_str() {
                "self" => SimplePathSegment::Self1,
                "super" => SimplePathSegment::Super,
                "crate" => SimplePathSegment::Crate,
                "$crate" => SimplePathSegment::DCrate, // Not possible tho
                _ => SimplePathSegment::Identifier(token.text.clone()),
            })
        } else {
            unexpected_eof(iter, errors);
            None
        }
    }
}
