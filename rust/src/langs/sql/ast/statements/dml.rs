use crate::langs::sql::ast::expression::SQLExpression;
use crate::langs::sql::ast::{SQLAst, SQLParserState};
use crate::langs::sql::lexer::SQLTokenKind;

#[derive(Debug, Clone)]
pub struct SQLInsert {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<SQLExpression>,
}

impl SQLInsert {
    pub fn new(state: &mut SQLParserState) -> Self {
        let mut s =
            Self { table_name: String::new(), columns: vec![], values: vec![] };

        state.advance();
        state.token_eat_text_ins("INTO");
        state.advance();
        if state.token_eat_kind(SQLTokenKind::Identifier) {
            s.table_name = state.token_text().unwrap().into();
            state.advance();
        }

        if state.token_eat_text_sen("(") {
            while state.not_at_end() {
                s.columns.push(state.token_text().unwrap().into());
                if state.token_eat_texts_sen(&[")", ";"]) {
                    break;
                }

                if state.token_expect_text_sen(",") {
                    if state.token_peek_at_text(1) == Some(")") {
                        state.record_trailing_comma();
                        state.advance();
                        break;
                    }
                }

                state.advance();
            }

            state.token_expect_text_sen(")");
            state.advance();
        }

        state.token_expect_text_ins("VALUES");
        state.advance();

        while state.not_at_end() {
            if state.token_eat_text_sen(";") {
                break;
            }

            state.token_expect_text_sen("(");
            while state.not_at_end() {
                s.values.push(SQLExpression::new(state));
                if state.token_eat_texts_sen(&[")", ";"]) {
                    break;
                }

                if state.token_expect_text_sen(",") {
                    if state.token_peek_at_text(1) == Some(")") {
                        state.record_trailing_comma();
                        state.advance();
                        break;
                    }
                }

                state.advance();
            }

            state.token_expect_text_sen(")");
            state.advance();

            if !state.token_eat_text_sen(",") {
                break;
            }
        }

        s
    }
}

impl From<SQLInsert> for SQLAst {
    fn from(value: SQLInsert) -> Self {
        Self::Insert(value)
    }
}

#[derive(Debug, Clone)]
pub struct SQLUpdate {
    pub table_name: String,
    pub assignments: Vec<SQLAssignment>,
    pub selection: Option<SQLExpression>,
}

impl SQLUpdate {
    pub fn new(state: &mut SQLParserState) -> Self {
        let mut s = Self {
            table_name: String::new(),
            assignments: vec![],
            selection: None,
        };

        state.advance();
        state.token_expect_kind(SQLTokenKind::Identifier);
        s.table_name = state.token_text().unwrap_or("").into();
        state.advance();
        state.token_expect_text_ins("SET");
        state.advance();
        while state.not_at_end() {
            if state.token_eat_texts_ins(&["WHERE", ";"]) {
                break;
            }

            s.assignments.push(SQLAssignment::new(state));
            if state.token_expect_text_ins(",") {
                state.advance();
                if state.token_eat_texts_ins(&["WHERE", ";"]) {
                    state.record_trailing_comma();
                    break;
                }

                state.seek_offset(-1);
            }

            state.advance();
        }

        if !state.token_eat_text_ins("WHERE") {
            return s;
        }

        state.advance();
        s.selection = Some(SQLExpression::new(state));
        s
    }
}

impl From<SQLUpdate> for SQLAst {
    fn from(value: SQLUpdate) -> Self {
        Self::Update(value)
    }
}

#[derive(Debug, Clone)]
pub struct SQLAssignment {
    pub column: String,
    pub value: SQLExpression,
}

impl SQLAssignment {
    pub fn new(state: &mut SQLParserState) -> Self {
        let column = state.token_text().unwrap_or("").into();
        state.advance();
        state.token_expect_text_ins("=");
        state.advance();
        let value = SQLExpression::new(state);
        SQLAssignment { column, value }
    }
}

#[derive(Debug, Clone)]
pub struct SQLDelete {
    pub table_name: String,
    pub selection: Option<SQLExpression>,
}

impl SQLDelete {
    pub fn new(state: &mut SQLParserState) -> Self {
        let mut s = Self { table_name: String::new(), selection: None };
        state.advance();
        state.token_expect_text_ins("FROM");
        state.advance();
        s.table_name = state.token_text().unwrap_or("").into();
        state.advance();
        state.token_expect_text_ins("WHERE");
        state.advance();
        s.selection = Some(SQLExpression::new(state));
        s
    }
}
