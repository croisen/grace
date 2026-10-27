use crate::api::GraceError;
use crate::api::ast::sql::expression::SqlExpression;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct SqlInsert {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<SqlExpression>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct SqlUpdate {
    pub table_name: String,
    pub assignments: Vec<SqlAssignment>,
    pub selection: Option<SqlExpression>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct SqlAssignment {
    pub column: String,
    pub value: SqlExpression,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct Delete {
    pub table_name: String,
    pub selection: Option<SqlExpression>,
}
