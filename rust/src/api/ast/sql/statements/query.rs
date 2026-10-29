use crate::api::GraceError;
use crate::api::ast::sql::expression::SQLExpression;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct SQLSelect {
    pub items: SQLSelectItems,
    pub from: Option<String>,
    pub joins: Vec<SQLJoinClause>,
    pub r#where: Option<SQLExpression>,
    pub group_by: Option<SQLGroupByClause>,
    pub having: Option<SQLHavingClause>,
    pub order_by: Option<SQLOrderByClause>,
    pub limit: Option<SQLLimitClause>,
}

#[derive(Debug, Clone)]
pub(crate) enum SQLSelectItems {
    Asterisk,
    Expr(SQLExpression, Option<SQLExpression>),
}

#[derive(Debug, Clone)]
pub(crate) struct SQLJoinClause {
    pub join: SQLJoinType,
    pub table: String,
    pub on: Option<SQLExpression>,
}

#[derive(Debug, Clone)]
pub(crate) enum SQLJoinType {
    Inner,
    Left,
    Right,
    Full,
}

#[derive(Debug, Clone)]
pub(crate) struct SQLGroupByClause {
    pub columns: Vec<SQLExpression>,
}

#[derive(Debug, Clone)]
pub(crate) struct SQLHavingClause {
    pub condition: SQLExpression,
}

#[derive(Debug, Clone)]
pub(crate) struct SQLOrderByClause {
    pub items: Vec<SQLOrderByItem>,
}

#[derive(Debug, Clone)]
pub(crate) struct SQLOrderByItem {
    pub expr: SQLExpression,
    pub direction: SQLOrderDirection,
}

#[derive(Debug, Clone)]
pub(crate) enum SQLOrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub(crate) struct SQLLimitClause {
    pub limit: SQLExpression,
    pub offset: Option<SQLExpression>,
}
