use crate::api::GraceError;
use crate::api::ast::sql::statements::query::SqlSelect;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) enum SqlExpression {
    Literal(String),
    Identifier(String),
    FunctionCall {
        name: String,
        args: Vec<SqlExpression>,
    },
    UnaryOp {
        op: String,
        expr: Box<SqlExpression>,
        is_prefix: bool,
    },
    BinaryOp {
        op: String,
        left: Box<SqlExpression>,
        right: Box<SqlExpression>,
    },
    Vector(Vec<SqlExpression>),
    InList {
        expr: Box<SqlExpression>,
        list: Vec<SqlExpression>,
        negated: bool,
    },
    Between {
        expr: Box<SqlExpression>,
        low: Box<SqlExpression>,
        high: Box<SqlExpression>,
        negated: bool,
    },
    SubQuery(Box<SqlSelect>),
}
