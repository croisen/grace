use crate::api::ast::rust::item::RustItem;
use crate::api::ast::rust::visibility::RustVisibility;
use crate::api::lexers::rust::{RustToken, RustTokenKind};

pub mod expression;
pub mod item;
pub mod pattern;
pub mod statement;
pub mod r#type;
pub mod visibility;

// VisItem in the Rust Reference book section 8.1
#[derive(Debug, Clone)]
pub(crate) struct RustAst {
    pub visibility: RustVisibility,
    pub item: RustItem,
}
