use crate::core::ast::rust::item::RustItem;
use crate::core::ast::rust::visibility::RustVisibility;
use crate::core::lexers::rust::{RustToken, RustTokenKind};

pub mod expression;
pub mod item;
pub mod pattern;
pub mod statement;
pub mod r#type;
pub mod visibility;

#[derive(Debug, Clone)]
pub struct RustAst {
    pub visibility: RustVisibility,
    pub item: RustItem,
}
