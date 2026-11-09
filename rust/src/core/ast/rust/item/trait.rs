use crate::core::ast::rust::item::alias::RustTypeAlias;
use crate::core::ast::rust::item::function::RustFunction;
use crate::core::ast::rust::r#type::generics::RustGeneric;
use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub struct RustTrait {
    pub name: String,
    pub generics: Option<RustGeneric>,
    pub items: Vec<RustTraitItem>,
}

#[derive(Debug, Clone)]
pub enum RustTraitItem {
    Method(RustFunction),
    Type(RustTypeAlias),
}
