use crate::core::ast::rust::r#type::RustType;
use crate::core::ast::rust::r#type::generics::RustGeneric;
use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub struct RustStruct {
    pub name: String,
    pub generics: Option<RustGeneric>,
    pub fields: Vec<RustStructField>,
}

#[derive(Debug, Clone)]
pub struct RustStructField {
    pub name: String,
    pub r#type: RustType,
}
