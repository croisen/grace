use crate::api::GraceError;
use crate::api::ast::sql::expression::SqlExpression;
use crate::api::ast::sql::statements::query::SqlSelect;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct SqlCreate {
    pub r#type: SqlCreateType,
    pub name: String,
    pub if_not_exists: bool,
    pub body: SqlCreateBody,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) enum SqlCreateType {
    TABLE,
    VIEW,
    DATABASE,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) enum SqlCreateBody {
    Table(Vec<SqlColumnDefinition>),
    View(SqlSelect),
    Index(String, Vec<String>, bool),
    Database,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct SqlColumnDefinition {
    pub name: String,
    pub r#type: String,
    pub constraints: Vec<SqlColumnRestraint>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) enum SqlColumnRestraint {
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
pub(crate) struct SqlDrop {
    pub r#type: SqlCreateType,
    pub name: String,
    pub if_exists: bool,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct SqlAlter {
    pub name: String,
    pub action: SqlAlterAction,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) enum SqlAlterAction {
    AddColumn(String, Option<String>),
    DropColumn(String),
    RenameTo(String),
}
