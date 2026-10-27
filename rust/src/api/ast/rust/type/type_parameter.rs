use crate::api::GraceError;
use crate::api::ast::rust::expression::RustExpression;
use crate::api::ast::rust::r#type::RustType;
use crate::api::ast::rust::r#type::lifetime::RustLifetime;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

pub(crate) struct RustTypeParameter {
    pub name: String,
    pub r#type: RustType,
    pub r#mut: bool,
}
