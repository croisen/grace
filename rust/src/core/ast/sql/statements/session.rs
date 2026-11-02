use crate::api::GraceError;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};

#[derive(Debug, Clone)]
pub(crate) struct SQLTransaction {
    pub action: SQLTransactionAction,
}

#[derive(Debug, Clone)]
pub(crate) enum SQLTransactionAction {
    Begin,
    Commit,
    Rollback,
}

#[derive(Debug, Clone)]
pub(crate) struct SQLSetNames {
    pub character_set: String,
    pub collation: Option<String>,
}
