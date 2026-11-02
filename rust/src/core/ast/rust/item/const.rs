use crate::api::ast::rust::expression::RustExpression;
use crate::api::ast::rust::r#type::RustType;
use crate::api::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub(crate) struct RustConst {
    pub name: String,
    pub r#type: RustType,
    pub expr: RustExpression,
}
