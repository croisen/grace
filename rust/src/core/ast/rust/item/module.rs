use crate::api::ast::rust::item::RustItem;
use crate::api::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub(crate) struct RustModule {
    pub name: String,
    pub items: Vec<RustItem>,
}
