use crate::core::ast::rust::item::RustItem;
use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub struct RustExtern {
    pub abi: Option<String>,
    pub items: Vec<RustItem>,
}
