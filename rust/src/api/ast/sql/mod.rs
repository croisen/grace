use crate::api::GraceError;
use crate::api::ast::sql::statements::ddl::{SqlAlter, SqlCreate, SqlDrop};
use crate::api::ast::sql::statements::dml::{Delete, SqlInsert, SqlUpdate};
use crate::api::ast::sql::statements::query::{
    SqlGroupByClause, SqlHavingClause, SqlJoinClause, SqlLimitClause,
    SqlOrderByClause, SqlSelectItems,
};
use crate::api::ast::sql::statements::session::{SqlSetNames, SqlTransaction};
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

pub mod expression;
pub mod statements;

#[derive(Debug, Clone)]
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

impl SqlAst {
    pub(crate) fn new(
        tokens: Vec<SQLToken>,
        mut errors: Vec<GraceError>,
    ) -> (Vec<Self>, Vec<GraceError>) {
        let mut ast = vec![];
        let mut iter = PrevIter::from(tokens);
        while let Some(token) = iter.next() {
            match token.kind {
                SQLTokenKind::StrictKeyword => {
                    if token.text.eq_ignore_ascii_case("create") {
                        ast.push(SqlCreate::new(&mut iter, &mut errors).into());
                    }
                }
                _ => {}
            }
        }

        (ast, errors)
    }
}
