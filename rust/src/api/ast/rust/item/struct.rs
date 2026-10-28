use crate::api::GraceError;
use crate::api::ast::rust::r#type::RustType;
use crate::api::ast::rust::r#type::generics::RustGeneric;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct RustStruct {
    pub name: String,
    pub generics: Option<RustGeneric>,
    pub fields: Vec<RustStructField>,
}

#[derive(Debug, Clone)]
pub(crate) struct RustStructField {
    pub name: String,
    pub r#type: RustType,
}
