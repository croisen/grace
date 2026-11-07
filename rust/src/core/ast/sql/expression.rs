use crate::core::ast::sql::{SQLAst, SQLParserState};
use crate::core::ast::sql::sstatements::query::SQLSelect;
use crate::core::lexers::sql::SQLTokenKind;
use crate::core::parser::pratt::{
    PrattAssociativity, PrattLanguage, parse_expr,
};

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

impl SQLExpression { 
    pub fn new<'a>(state: &mut SQLParserState<'a>)-> Self {
        parse_expr::<Self>(state, 0)
    }
}

impl<'a> PrattLanguage<'a> for SQLExpression {
    type Root = SQLAst;
    type TokenKind = SQLTokenKind;
}
