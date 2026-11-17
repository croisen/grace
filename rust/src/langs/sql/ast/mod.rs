use crate::core::parser::output::ParserOutput;
use crate::core::parser::state::ParserState;
use crate::langs::sql::ast::statements::ddl::{SQLAlter, SQLCreate, SQLDrop};
use crate::langs::sql::ast::statements::dml::{Delete, SQLInsert, SQLUpdate};
use crate::langs::sql::ast::statements::query::{
    SQLGroupByClause, SQLHavingClause, SQLJoinClause, SQLLimitClause,
    SQLOrderByClause, SQLSelect,
};
use crate::langs::sql::ast::statements::session::{
    SQLSetNames, SQLTransaction,
};
use crate::langs::sql::lexer::SQLTokenKind;

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

    Select(SQLSelect),
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
                let d = SQLDrop::new(&mut state);
                state.root_push(d);
            } else if state.token_eat_text_ins("alter") {
                state.record_unexpected_token();
            } else if state.token_eat_text_ins("insert") {
                state.record_unexpected_token();
            } else if state.token_eat_text_ins("update") {
                state.record_unexpected_token();
            } else if state.token_eat_text_ins("delete") {
                state.record_unexpected_token();
            } else if state.token_eat_text_ins("select") {
                let s = SQLSelect::new(&mut state);
                state.root_push(s);
            } else {
                state.record_unexpected_token();
            }

            state.token_expect_text_sen(";");
            state.advance();
        }

        state.into()
    }
}
