use crate::core::ast::rust::expression::RustExpression;
use crate::core::ast::rust::r#type::RustType;
use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub struct RustConst {
    pub name: String,
    pub r#type: RustType,
    pub expr: RustExpression,
}
