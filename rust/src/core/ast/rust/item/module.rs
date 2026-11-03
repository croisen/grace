use crate::core::ast::rust::item::RustItem;
use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub struct RustModule {
    pub name: String,
    pub items: Vec<RustItem>,
}
