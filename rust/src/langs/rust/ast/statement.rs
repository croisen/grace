use crate::core::parser::pratt::Token2;
use crate::langs::rust::ast::expression::RustExpression;
use crate::langs::rust::ast::item::RustItem;
use crate::langs::rust::ast::r#type::RustType;

#[derive(Debug, Clone)]
pub enum RustStatement {
    Let {
        mutable: bool,
        name: Token2,
        r#type: Option<RustType>,
        init: Option<RustExpression>,
    },
    Expression {
        expression: RustExpression,
        has_semicolon: bool,
    },
    Return(Option<RustExpression>),
    Break,
    Continue,
    Item(RustItem),
}
