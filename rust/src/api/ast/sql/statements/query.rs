use crate::api::GraceError;
use crate::api::ast::sql::expression::SqlExpression;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct SqlSelect {
    pub items: SqlSelectItems,
    pub from: Option<String>,
    pub joins: Vec<SqlJoinClause>,
    pub r#where: Option<SqlExpression>,
    pub group_by: Option<SqlGroupByClause>,
    pub having: Option<SqlHavingClause>,
    pub order_by: Option<SqlOrderByClause>,
    pub limit: Option<SqlLimitClause>,
}

#[derive(Debug, Clone)]
pub(crate) enum SqlSelectItems {
    Asterisk,
    Expr(SqlExpression, Option<SqlExpression>),
}

#[derive(Debug, Clone)]
pub(crate) struct SqlJoinClause {
    pub join: SqlJoinType,
    pub table: String,
    pub on: Option<SqlExpression>,
}

#[derive(Debug, Clone)]
pub(crate) enum SqlJoinType {
    Inner,
    Left,
    Right,
    Full,
}

#[derive(Debug, Clone)]
pub(crate) struct SqlGroupByClause {
    pub columns: Vec<SqlExpression>,
}

#[derive(Debug, Clone)]
pub(crate) struct SqlHavingClause {
    pub condition: SqlExpression,
}

#[derive(Debug, Clone)]
pub(crate) struct SqlOrderByClause {
    pub items: Vec<SqlOrderByItem>,
}

#[derive(Debug, Clone)]
pub(crate) struct SqlOrderByItem {
    pub expr: SqlExpression,
    pub direction: SqlOrderDirection,
}

#[derive(Debug, Clone)]
pub(crate) enum SqlOrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub(crate) struct SqlLimitClause {
    pub limit: SqlExpression,
    pub offset: Option<SqlExpression>,
}
