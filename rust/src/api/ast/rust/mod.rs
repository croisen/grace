use crate::api::GraceError;
use crate::api::ast::rust::item::RustItem;
use crate::api::ast::rust::visibility::RustVisibility;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

pub mod expression;
pub mod item;
pub mod pattern;
pub mod statement;
pub mod r#type;
pub mod visibility;

// VisItem in the Rust Reference book section 8.1
pub(crate) struct RustAst {
    pub visibility: RustVisibility,
    pub item: RustItem,
}
