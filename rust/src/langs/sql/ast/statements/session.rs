use crate::core::parser::pratt::Token2;

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
    pub character_set: Token2,
    pub collation: Option<Token2>,
}
