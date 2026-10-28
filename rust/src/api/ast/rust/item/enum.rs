use crate::api::GraceError;
use crate::api::ast::rust::item::r#struct::RustStructField;
use crate::api::ast::rust::r#type::RustType;
use crate::api::ast::rust::r#type::generics::RustGeneric;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct RustEnum {
    pub name: String,
    pub generics: Option<RustGeneric>,
    pub variants: Vec<RustEnumVariant>,
}

#[derive(Debug, Clone)]
pub(crate) enum RustEnumVariant {
    Struct(String, Vec<RustStructField>),
    Tuple(String, Vec<RustType>),
    Unit(String),
}
