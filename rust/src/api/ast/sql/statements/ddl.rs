use crate::api::GraceError;
use crate::api::ast::sql::expression::SqlExpression;
use crate::api::ast::sql::statements::query::SqlSelect;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct SqlCreate {
    pub r#type: SqlCreateType,
    pub name: String,
    pub if_not_exists: bool,
    pub body: SqlCreateBody,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub enum SqlCreateType {
    TABLE,
    VIEW,
    DATABASE,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub enum SqlCreateBody {
    Table(Vec<SqlColumnDefinition>),
    View(SqlSelect),
    Index(String, Vec<String>, bool),
    Database,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct SqlColumnDefinition {
    pub name: String,
    pub r#type: String,
    pub constraints: Vec<SqlColumnRestraint>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub enum SqlColumnRestraint {
    PrimaryKey,
    NotNull,
    Nullable,
    Unique,
    Default(SqlExpression),
    Check(SqlExpression),
    AutoIncrement,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct SqlDrop {
    pub r#type: SqlCreateType,
    pub name: String,
    pub if_exists: bool,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct SqlAlter {
    pub name: String,
    pub action: SqlAlterAction,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub enum SqlAlterAction {
    AddColumn(String, Option<String>),
    DropColumn(String),
    RenameTo(String),
}
