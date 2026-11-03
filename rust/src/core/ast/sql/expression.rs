use crate::core::ast::sql::statements::query::SQLSelect;
use crate::core::lexers::sql::{SQLToken, SQLTokenKind};

#[derive(Debug, Clone)]
pub enum SQLExpression {
    Literal(String),
    Identifier(String),
    FunctionCall {
        name: String,
        args: Vec<SQLExpression>,
    },
    UnaryOp {
        op: String,
        expr: Box<SQLExpression>,
        is_prefix: bool,
    },
    BinaryOp {
        op: String,
        left: Box<SQLExpression>,
        right: Box<SQLExpression>,
    },
    Vector(Vec<SQLExpression>),
    InList {
        expr: Box<SQLExpression>,
        list: Vec<SQLExpression>,
        negated: bool,
    },
    Between {
        expr: Box<SQLExpression>,
        low: Box<SQLExpression>,
        high: Box<SQLExpression>,
        negated: bool,
    },
    SubQuery(Box<SQLSelect>),
}
