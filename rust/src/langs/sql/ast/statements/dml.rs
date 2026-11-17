use crate::langs::sql::ast::expression::SQLExpression;

#[derive(Debug, Clone)]
pub struct SQLInsert {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<SQLExpression>,
}

#[derive(Debug, Clone)]
pub struct SQLUpdate {
    pub table_name: String,
    pub assignments: Vec<SQLAssignment>,
    pub selection: Option<SQLExpression>,
}

#[derive(Debug, Clone)]
pub struct SQLAssignment {
    pub column: String,
    pub value: SQLExpression,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub table_name: String,
    pub selection: Option<SQLExpression>,
}
