use crate::core::parser::pratt::Token2;
use crate::langs::rust::ast::expression::RustExpression;
use crate::langs::rust::ast::r#type::RustType;

#[derive(Debug, Clone)]
pub struct RustStatic {
    pub r#mut: bool,
    pub name: Token2,
    pub r#type: RustType,
    pub expr: RustExpression,
}
