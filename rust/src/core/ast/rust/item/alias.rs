use crate::core::ast::rust::r#type::RustType;
use crate::core::ast::rust::r#type::generics::RustGeneric;
use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub struct RustTypeAlias {
    pub name: String,
    pub generics: Option<RustGeneric>,
    pub r#type: RustType,
}
