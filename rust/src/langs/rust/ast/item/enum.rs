use crate::core::parser::pratt::Token2;
use crate::langs::rust::ast::item::r#struct::RustStructField;
use crate::langs::rust::ast::r#type::RustType;
use crate::langs::rust::ast::r#type::generics::RustGeneric;

#[derive(Debug, Clone)]
pub struct RustEnum {
    pub name: Token2,
    pub generics: Option<RustGeneric>,
    pub variants: Vec<RustEnumVariant>,
}

#[derive(Debug, Clone)]
pub enum RustEnumVariant {
    Struct(Token2, Vec<RustStructField>),
    Tuple(Token2, Vec<RustType>),
    Unit(Token2),
}
