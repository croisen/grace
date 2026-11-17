use crate::langs::rust::ast::r#type::RustType;
use crate::langs::rust::ast::r#type::generics::RustGeneric;

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
