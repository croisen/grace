use crate::langs::rust::ast::r#type::RustType;
use crate::langs::rust::ast::r#type::generics::RustGeneric;

#[derive(Debug, Clone)]
pub struct RustTypeAlias {
    pub name: String,
    pub generics: Option<RustGeneric>,
    pub r#type: RustType,
}
