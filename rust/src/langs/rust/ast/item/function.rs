use crate::langs::rust::ast::expression::RustBlock;
use crate::langs::rust::ast::r#type::RustType;
use crate::langs::rust::ast::r#type::generics::RustGeneric;
use crate::langs::rust::ast::r#type::parameter::RustParameter;

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
