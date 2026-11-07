use crate::core::ast::sql::statements::ddl::{SQLAlter, SQLCreate, SQLDrop};
use crate::core::ast::sql::statements::dml::{Delete, SQLInsert, SQLUpdate};
use crate::core::ast::sql::statements::query::{
    SQLGroupByClause, SQLHavingClause, SQLJoinClause, SQLLimitClause,
    SQLOrderByClause, SQLSelectItem,
};
use crate::core::ast::sql::statements::session::{SQLSetNames, SQLTransaction};
use crate::core::lexers::sql::SQLTokenKind;
use crate::core::parser::output::ParserOutput;
use crate::core::parser::state::ParserState;

pub mod expression;
pub mod statements;

pub type SQLParserState<'a> = ParserState<'a, SQLTokenKind, SQLAst>;

#[derive(Debug, Clone)]
pub enum SQLAst {
    Create(SQLCreate),
    Drop(SQLDrop),
    Alter(SQLAlter),

    Insert(SQLInsert),
    Update(SQLUpdate),
    Delete(Delete),

    Select(SQLSelectItem),
    Join(SQLJoinClause),
    GroupBy(SQLGroupByClause),
    Having(SQLHavingClause),
    OrderBy(SQLOrderByClause),
    Limit(SQLLimitClause),

    Transaction(SQLTransaction),
    SetNames(SQLSetNames),
}

impl SQLAst {
    pub fn new<'a>(source: &'a str) -> ParserOutput<Self> {
        let mut state = ParserState::<'a, SQLTokenKind, Self>::new(source);
        while state.not_at_end() {
            if !state.token_expect_kind(SQLTokenKind::StrictKeyword) {
                state.advance();
            }

            if state.token_eat_text_ins("create") {
                let c = SQLCreate::new(&mut state);
                state.root_push(c);
            } else if state.token_eat_text_ins("drop") {
                state.record_unexpected_token();
            } else if state.token_eat_text_ins("alter") {
                state.record_unexpected_token();
            } else if state.token_eat_text_ins("insert") {
                state.record_unexpected_token();
            } else if state.token_eat_text_ins("update") {
                state.record_unexpected_token();
            } else if state.token_eat_text_ins("delete") {
                state.record_unexpected_token();
            } else if state.token_eat_text_ins("select") {
                state.record_unexpected_token();
            } else {
                state.record_unexpected_token();
            }

            state.token_expect_text_sen(";");
            state.advance();
        }

        state.into()
    }
}
