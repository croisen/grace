use crate::core::parser::pratt::Token2;
use crate::langs::rust::ast::r#type::RustType;

#[derive(Debug, Clone)]
pub struct RustParameter {
    pub name: Token2,
    pub r#type: RustType,
    pub r#mut: bool,
}
