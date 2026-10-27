use crate::api::GraceError;
use crate::api::ast::rust::expression::RustExpression;
use crate::api::ast::rust::item::RustItem;
use crate::api::ast::rust::r#type::RustType;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

pub(crate) enum RustStatement {
    Let(bool, String, Option<RustType>, Option<RustExpression>),
    Expression(RustExpression, bool),
    Return(Option<RustExpression>),
    Break,
    Continue,
    Item(RustItem),
}
