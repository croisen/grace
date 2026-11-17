use crate::langs::rust::ast::expression::RustExpression;
use crate::langs::rust::ast::item::RustItem;
use crate::langs::rust::ast::r#type::RustType;

#[derive(Debug, Clone)]
pub enum RustStatement {
    Let(bool, String, Option<RustType>, Option<RustExpression>),
    Expression(RustExpression, bool),
    Return(Option<RustExpression>),
    Break,
    Continue,
    Item(RustItem),
}
