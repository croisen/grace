use crate::core::ast::rust::expression::RustBlock;
use crate::core::ast::rust::r#type::RustType;
use crate::core::ast::rust::r#type::generics::RustGeneric;
use crate::core::ast::rust::r#type::parameter::RustParameter;
use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub struct RustFunction {
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
