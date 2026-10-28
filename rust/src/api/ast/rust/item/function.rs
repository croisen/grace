use crate::api::GraceError;
use crate::api::ast::rust::expression::RustBlock;
use crate::api::ast::rust::r#type::RustType;
use crate::api::ast::rust::r#type::generics::RustGeneric;
use crate::api::ast::rust::r#type::parameter::RustParameter;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct RustFunction {
    pub name: String,
    pub generics: Vec<RustGeneric>,
    pub params: Vec<RustParameter>,
    pub r#return: Option<RustType>,
    pub r#async: bool,
    pub r#unsafe: bool,
    pub r#extern: bool,
    pub r#const: bool,
    pub body: RustBlock,
}
