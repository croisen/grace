use crate::api::GraceError;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct SqlTransaction {
    pub action: SqlTransactionAction,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) enum SqlTransactionAction {
    Begin,
    Commit,
    Rollback,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct SqlSetNames {
    pub character_set: String,
    pub collation: Option<String>,
}
