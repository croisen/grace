use crate::api::GraceError;
use crate::api::ast::sql::statements::ddl::{SQLAlter, SQLCreate, SQLDrop};
use crate::api::ast::sql::statements::dml::{Delete, SQLInsert, SQLUpdate};
use crate::api::ast::sql::statements::query::{
    SQLGroupByClause, SQLHavingClause, SQLJoinClause, SQLLimitClause,
    SQLOrderByClause, SQLSelectItems,
};
use crate::api::ast::sql::statements::session::{SQLSetNames, SQLTransaction};
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::macros::iter_unwrap;

pub mod expression;
pub mod statements;

#[derive(Debug, Clone)]
pub(crate) enum SQLAst {
    Create(SQLCreate),
    Drop(SQLDrop),
    Alter(SQLAlter),

    Insert(SQLInsert),
    Update(SQLUpdate),
    Delete(Delete),

    Select(SQLSelectItems),
    Join(SQLJoinClause),
    GroupBy(SQLGroupByClause),
    Having(SQLHavingClause),
    OrderBy(SQLOrderByClause),
    Limit(SQLLimitClause),

    Transaction(SQLTransaction),
    SetNames(SQLSetNames),
}

impl SQLAst {}

impl From<SQLCreate> for SQLAst {
    fn from(value: SQLCreate) -> Self {
        Self::Create(value)
    }
}
