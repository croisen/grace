use crate::api::GraceError;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
// Statements in the Rust Reference book section 8.1
pub enum RustAst {
    Item,
    Let,
    Expression,
    MacroItem,
    Unknown,
}
