use crate::core::parser::output::ParserOutput;
use crate::core::parser::state::ParserState;
use crate::langs::sql::ast::statements::ddl::{SQLAlter, SQLCreate, SQLDrop};
use crate::langs::sql::ast::statements::dml::{
    SQLDelete, SQLInsert, SQLUpdate,
};
use crate::langs::sql::ast::statements::query::SQLSelect;
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
    Delete(SQLDelete),

    Select(SQLSelect),
    Truncate(crate::core::parser::pratt::Token2),

    // Do we really use this?
    Transaction(SQLTransaction),
    SetNames(SQLSetNames),
}

impl SQLAst {
    pub fn new(source: &str) -> ParserOutput<Self> {
        let mut state = ParserState::<SQLTokenKind, Self>::new(source);
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
                let a = SQLAlter::new(&mut state);
                state.root_push(a);
            } else if state.token_eat_text_ins("insert") {
                let i = SQLInsert::new(&mut state);
                state.root_push(i);
            } else if state.token_eat_text_ins("update") {
                let u = SQLUpdate::new(&mut state);
                state.root_push(u);
            } else if state.token_eat_text_ins("delete") {
                let d = SQLDelete::new(&mut state);
                state.root_push(d);
            } else if state.token_eat_text_ins("select") {
                let s = SQLSelect::new(&mut state);
                state.root_push(s);
            } else if state.token_eat_text_ins("truncate") {
                state.advance();
                state.token_take_text_ins("TABLE");
                let table =
                    state.token_current().map(Into::into).unwrap_or_default();
                state.advance();
                state.root_push(Self::Truncate(table));
            } else {
                state.record_unexpected_token();
            }

            state.token_expect_text_sen(";");
            state.advance();
        }

        state.into()
    }
}
