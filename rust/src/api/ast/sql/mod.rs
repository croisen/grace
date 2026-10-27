use crate::api::ast::sql::statements::ddl::{SqlAlter, SqlCreate, SqlDrop};
use crate::api::ast::sql::statements::dml::{Delete, SqlInsert, SqlUpdate};
use crate::api::ast::sql::statements::query::{
    SqlGroupByClause, SqlHavingClause, SqlJoinClause, SqlLimitClause,
    SqlOrderByClause, SqlSelectItems,
};
use crate::api::ast::sql::statements::session::{SqlSetNames, SqlTransaction};

pub mod expression;
pub mod statements;

pub(crate) enum SqlAst {
    Create(SqlCreate),
    Drop(SqlDrop),
    Alter(SqlAlter),

    Insert(SqlInsert),
    Update(SqlUpdate),
    Delete(Delete),

    Select(SqlSelectItems),
    Join(SqlJoinClause),
    GroupBy(SqlGroupByClause),
    Having(SqlHavingClause),
    OrderBy(SqlOrderByClause),
    Limit(SqlLimitClause),

    Transaction(SqlTransaction),
    SetNames(SqlSetNames),
}
