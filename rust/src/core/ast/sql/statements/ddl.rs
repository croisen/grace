use std::collections::HashSet;

use crate::api::GraceError;
use crate::api::ast::sql::SQLAst;
use crate::api::ast::sql::expression::SQLExpression;
use crate::api::ast::sql::statements::query::SQLSelect;
use crate::api::ast::unexpected_eof;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::macros::iter_unwrap;

#[derive(Debug, Clone)]
pub(crate) struct SQLCreate {
    pub name: String,
    pub if_not_exists: bool,
    pub body: SQLCreateBody,
}

#[derive(Debug, Clone)]
pub(crate) enum SQLCreateBody {
    Table(Vec<SQLColumnDefinition>),
    View(SQLSelect),
    Index_(String, Vec<String>, bool), // freezed.dart uses this name for tuple enums?
    Database,
}

#[derive(Debug, Clone)]
pub(crate) struct SQLColumnDefinition {
    pub name: String,
    pub r#type: String,
    pub constraints: Vec<SQLColumnRestraint>,
}

#[derive(Debug, Clone)]
pub(crate) enum SQLColumnRestraint {
    PrimaryKey,
    ForeignKey,
    NotNull,
    Nullable,
    Unique,
    Default(SQLExpression),
    Check(SQLExpression),
    AutoIncrement,
}

#[derive(Debug, Clone)]
pub(crate) struct SQLDrop {
    pub r#type: SQLDropType,
    pub name: String,
    pub if_exists: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum SQLDropType {
    Table,
    View,
    Index,
    Database,
}

#[derive(Debug, Clone)]
pub(crate) struct SQLAlter {
    pub name: String,
    pub action: Option<SQLAlterAction>,
}

#[derive(Debug, Clone)]
pub(crate) enum SQLAlterAction {
    AddColumn(String, Option<String>),
    DropColumn(String),
    RenameTo(String),
}
