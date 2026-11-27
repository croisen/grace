use crate::core::parser::pratt::Token2;
use crate::langs::rust::ast::r#type::RustType;
use crate::langs::rust::ast::r#type::generics::RustGeneric;

#[derive(Debug, Clone)]
pub struct RustStruct {
    pub name: Token2,
    pub generics: Option<RustGeneric>,
    pub fields: Vec<RustStructField>,
}

#[derive(Debug, Clone)]
pub struct RustStructField {
    pub name: Token2,
    pub r#type: RustType,
}
