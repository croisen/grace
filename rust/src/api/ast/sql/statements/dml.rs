use crate::api::GraceError;
use crate::api::ast::sql::expression::SQLExpression;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct SQLInsert {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<SQLExpression>,
}

#[derive(Debug, Clone)]
pub(crate) struct SQLUpdate {
    pub table_name: String,
    pub assignments: Vec<SQLAssignment>,
    pub selection: Option<SQLExpression>,
}

#[derive(Debug, Clone)]
pub(crate) struct SQLAssignment {
    pub column: String,
    pub value: SQLExpression,
}

#[derive(Debug, Clone)]
pub(crate) struct Delete {
    pub table_name: String,
    pub selection: Option<SQLExpression>,
}
