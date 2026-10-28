use crate::api::GraceError;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct SqlTransaction {
    pub action: SqlTransactionAction,
}

#[derive(Debug, Clone)]
pub(crate) enum SqlTransactionAction {
    Begin,
    Commit,
    Rollback,
}

#[derive(Debug, Clone)]
pub(crate) struct SqlSetNames {
    pub character_set: String,
    pub collation: Option<String>,
}
