use crate::langs::rust::ast::r#type::RustType;
use crate::langs::rust::ast::r#type::generics::RustGeneric;

#[derive(Debug, Clone)]
pub struct RustTypeAlias {
    pub name: Token2,
    pub generics: Option<RustGeneric>,
    pub r#type: RustType,
}
use crate::core::parser::pratt::Token2;
