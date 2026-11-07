use crate::core::ast::sql::SQLParserState;
use crate::core::ast::sql::expression::SQLExpression;
use crate::core::lexers::sql::{SQLToken, SQLTokenKind};

#[derive(Debug, Clone)]
pub struct SQLSelect {
    pub items: SQLSelectItems,
    pub from: Option<String>,
    pub joins: Vec<SQLJoinClause>,
    pub r#where: Option<SQLExpression>,
    pub group_by: Option<SQLGroupByClause>,
    pub having: Option<SQLHavingClause>,
    pub order_by: Option<SQLOrderByClause>,
    pub limit: Option<SQLLimitClause>,
}

impl SQLSelect {
    pub fn new(state: &mut SQLParserState) -> Self {
        let mut s = Self {
            items: vec![],
            from: None,
            joins: vec![],
            r#where: None,
            group_by: None,
            having: None,
            order_by: None,
            limit: None,
        };

        s
    }
}

#[derive(Debug, Clone)]
pub enum SQLSelectItem {
    Asterisk,
    Expr(SQLExpression, Option<SQLExpression>),
}

impl SQLSelectItem {
    pub fn new(state: &mut SQLParserState) -> Self {
        Self::Asterisk
    }
}

#[derive(Debug, Clone)]
pub struct SQLJoinClause {
    pub join: SQLJoinType,
    pub table: String,
    pub on: Option<SQLExpression>,
}

#[derive(Debug, Clone)]
pub enum SQLJoinType {
    Inner,
    Left,
    Right,
    Full,
}

#[derive(Debug, Clone)]
pub struct SQLGroupByClause {
    pub columns: Vec<SQLExpression>,
}

#[derive(Debug, Clone)]
pub struct SQLHavingClause {
    pub condition: SQLExpression,
}

#[derive(Debug, Clone)]
pub struct SQLOrderByClause {
    pub items: Vec<SQLOrderByItem>,
}

#[derive(Debug, Clone)]
pub struct SQLOrderByItem {
    pub expr: SQLExpression,
    pub direction: SQLOrderDirection,
}

#[derive(Debug, Clone)]
pub enum SQLOrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct SQLLimitClause {
    pub limit: SQLExpression,
    pub offset: Option<SQLExpression>,
}
