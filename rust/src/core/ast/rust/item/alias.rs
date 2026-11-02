use crate::api::ast::rust::r#type::RustType;
use crate::api::ast::rust::r#type::generics::RustGeneric;
use crate::api::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub(crate) struct RustTypeAlias {
    pub name: String,
    pub generics: Option<RustGeneric>,
    pub r#type: RustType,
}
