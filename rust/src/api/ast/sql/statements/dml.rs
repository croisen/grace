use crate::api::GraceError;
use crate::api::ast::sql::expression::SqlExpression;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct SqlInsertStatement {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<SqlExpression>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct SqlUpdateStatement {
    pub table_name: String,
    pub assignments: Vec<SqlAssignment>,
    pub selection: Option<SqlExpression>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct SqlAssignment {
    pub column: String,
    pub value: SqlExpression,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct DeleteStatement {
    pub table_name: String,
    pub selection: Option<SqlExpression>,
}
