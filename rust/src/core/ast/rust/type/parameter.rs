use crate::api::ast::rust::expression::RustExpression;
use crate::api::ast::rust::r#type::RustType;
use crate::api::ast::rust::r#type::lifetime::RustLifetime;
use crate::api::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub(crate) struct RustParameter {
    pub name: String,
    pub r#type: RustType,
    pub r#mut: bool,
}
