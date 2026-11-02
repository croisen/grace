use crate::api::ast::rust::item::RustItem;
use crate::api::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub(crate) struct RustExtern {
    pub abi: Option<String>,
    pub items: Vec<RustItem>,
}
