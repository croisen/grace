use crate::api::GraceError;
use crate::api::ast::rust::simplepath::SimplePath;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::macros::skip_match;
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone, PartialEq)]
#[flutter_rust_bridge::frb(ignore)]
pub enum Visibility {
    Pub,
    PubCrate,
    PubSelf,
    PubSuper,
    PubIn(SimplePath),
}

impl Visibility {
    pub fn new(
        iter: &mut PrevIter<'_, RustToken>,
        errors: &mut Vec<GraceError>,
    ) -> Option<Self> {
        let token = iter.current();
        if token.is_none() {
            return None;
        }

        let token = unsafe { token.unwrap_unchecked() };
        if token.kind != RustTokenKind::StrictKeyword {
            return None;
        }

        if token.text != "pub" {
            return None;
        }

        Some(Self::normal_pub(iter, errors))
    }

    fn normal_pub(
        iter: &mut PrevIter<'_, RustToken>,
        errors: &mut Vec<GraceError>,
    ) -> Self {
        let token = iter.next();
        if token.is_none() {
            return Self::Pub;
        }

        let token = unsafe { token.unwrap_unchecked() };
        if token.kind != RustTokenKind::Pairs {
            let p = iter.prev();
            return Self::Pub;
        }

        if token.text != "(" {
            println!("--{token:?}");
            let _ = iter.prev();
            return Self::Pub;
        }

        Self::worded_pub(iter, errors)
    }

    fn worded_pub(
        iter: &mut PrevIter<'_, RustToken>,
        errors: &mut Vec<GraceError>,
    ) -> Self {
        let token = iter.next();
        if token.is_none() {
            return Self::Pub;
        }

        let token = unsafe { token.unwrap_unchecked() };
        if !matches!(
            token.kind,
            RustTokenKind::Identifier
                | RustTokenKind::StrictKeyword
                | RustTokenKind::MemberAccess
        ) {
            GraceError::syntax_error(
                errors,
                token,
                format!(
                    "expected identifier, self, crate, super, :: found `{}`",
                    token.text
                ),
            );

            skip_match!(iter, RustTokenKind::Pairs);
            return Self::Pub;
        }

        if token.text == "super" {
            skip_match!(iter, RustTokenKind::Pairs);
            Self::PubSuper
        } else if token.text == "crate" {
            skip_match!(iter, RustTokenKind::Pairs);
            Self::PubCrate
        } else if token.text == "self" {
            skip_match!(iter, RustTokenKind::Pairs);
            Self::PubSelf
        } else {
            let p = SimplePath::new(iter, errors);
            skip_match!(iter, RustTokenKind::Pairs);
            Self::PubIn(p)
        }
    }
}
