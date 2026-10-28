use crate::api::GraceError;
use crate::api::ast::rust::expression::RustExpression;
use crate::api::ast::rust::r#type::RustType;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct RustStatic {
    pub r#mut: bool,
    pub name: String,
    pub r#type: RustType,
    pub expr: RustExpression,
}
