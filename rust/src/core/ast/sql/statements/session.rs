use crate::core::lexers::sql::{SQLToken, SQLTokenKind};

#[derive(Debug, Clone)]
pub struct SQLTransaction {
    pub action: SQLTransactionAction,
}

#[derive(Debug, Clone)]
pub enum SQLTransactionAction {
    Begin,
    Commit,
    Rollback,
}

#[derive(Debug, Clone)]
pub struct SQLSetNames {
    pub character_set: String,
    pub collation: Option<String>,
}
