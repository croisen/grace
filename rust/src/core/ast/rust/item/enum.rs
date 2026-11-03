use crate::core::ast::rust::item::r#struct::RustStructField;
use crate::core::ast::rust::r#type::RustType;
use crate::core::ast::rust::r#type::generics::RustGeneric;
use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub struct RustEnum {
    pub name: String,
    pub generics: Option<RustGeneric>,
    pub variants: Vec<RustEnumVariant>,
}

#[derive(Debug, Clone)]
pub enum RustEnumVariant {
    Struct(String, Vec<RustStructField>),
    Tuple(String, Vec<RustType>),
    Unit(String),
}
