use crate::langs::rust::ast::item::r#struct::RustStructField;
use crate::langs::rust::ast::r#type::RustType;
use crate::langs::rust::ast::r#type::generics::RustGeneric;

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
