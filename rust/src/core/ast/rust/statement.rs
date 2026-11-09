use crate::core::ast::rust::expression::RustExpression;
use crate::core::ast::rust::item::RustItem;
use crate::core::ast::rust::r#type::RustType;
use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub enum RustStatement {
    Let(bool, String, Option<RustType>, Option<RustExpression>),
    Expression(RustExpression, bool),
    Return(Option<RustExpression>),
    Break,
    Continue,
    Item(RustItem),
}
