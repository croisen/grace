use crate::api::GraceError;
use crate::api::ast::sql::statements::ddl::{SQLAlter, SQLCreate, SQLDrop};
use crate::api::ast::sql::statements::dml::{Delete, SQLInsert, SQLUpdate};
use crate::api::ast::sql::statements::query::{
    SQLGroupByClause, SQLHavingClause, SQLJoinClause, SQLLimitClause,
    SQLOrderByClause, SQLSelectItems,
};
use crate::api::ast::sql::statements::session::{SQLSetNames, SQLTransaction};
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

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

impl SQLAst {
    pub(crate) fn new(source: impl AsRef<str>) -> (Vec<Self>, Vec<GraceError>) {
        let (tokens, mut errors) = SQLToken::tokenize(source.as_ref());
        let mut ast = vec![];
        let mut iter = PrevIter::from(tokens);
        while let Some(token) = iter.next() {
            match token.kind {
                SQLTokenKind::StrictKeyword => {
                    if token.text.eq_ignore_ascii_case("create") {
                        ast.push(SQLCreate::new(&mut iter, &mut errors).into());
                    }
                }
                _ => {}
            }
        }

        (ast, errors)
    }
}
