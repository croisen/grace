use crate::api::GraceError;
use crate::api::ast::rust::statement::LetStatement;
use crate::api::ast::rust::visitem::VisItem;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

pub mod expression;
pub mod item;
pub mod paths;
pub mod patterns;
pub mod safety;
pub mod statement;
pub mod use_declaration;
pub mod visibility;
pub mod visitem;

#[derive(Debug, Clone, PartialEq)]
#[flutter_rust_bridge::frb(ignore)]
// Statements in the Rust Reference book section 8.1
pub enum RustAst {
    Item(VisItem),
    Let(LetStatement),
    Expression,
    // MacroItem(MacroItem),
    Unknown,
}

impl RustAst {
    pub fn new(
        tokens: Vec<RustToken>,
        mut errors: Vec<GraceError>,
    ) -> (Vec<Self>, Vec<GraceError>) {
        let mut iter = PrevIter::from(tokens);
        let mut ast = vec![];
        while let Some(token) = iter.next() {
            match token.kind {
                RustTokenKind::Delimiters => {
                    if token.text == ";" {
                        continue;
                    }

                    GraceError::syntax_error(
                        &mut errors,
                        token,
                        "expected an item",
                    );
                }
                RustTokenKind::StrictKeyword if (token.text == "let") => {}
                _ => {
                    ast.append(
                        &mut VisItem::new(&mut iter, &mut errors)
                            .into_iter()
                            .map(|u| u.into())
                            .collect(),
                    );
                }
            }
        }

        println!();
        errors.sort();
        (ast, errors)
    }
}

impl Default for RustAst {
    fn default() -> Self {
        Self::Unknown
    }
}

fn unexpected_eof(
    iter: &mut PrevIter<'_, RustToken>,
    errors: &mut Vec<GraceError>,
) {
    iter.seek_end();
    let p = iter.prev().unwrap();
    GraceError::syntax_error(errors, p, "unexpected EOF");
    let _ = iter.next();
}
