use crate::langs::sql::ast::expression::SQLExpression;
use crate::langs::sql::ast::{SQLAst, SQLParserState};
use crate::langs::sql::lexer::SQLTokenKind;

#[derive(Debug, Clone)]
pub struct SQLInsert {
    pub table_name: Token2,
    pub columns: Vec<Token2>,
    pub values: Vec<SQLExpression>,
}

impl SQLInsert {
    pub fn new(state: &mut SQLParserState) -> Self {
        let mut s = Self {
            table_name: Token2::default(),
            columns: vec![],
            values: vec![],
        };

        state.advance();
        state.token_take_text_ins("INTO");
        if state.token_eat_kind(SQLTokenKind::Identifier) {
            s.table_name = state.token_current().unwrap().into();
            state.advance();
        }

        if state.token_take_text_sen("(") {
            while state.not_at_end() && !state.token_eat_text_sen(")") {
                if state.token_eat_text_sen(",") {
                    state.record_unexpected_token();
                    state.advance();
                    continue;
                }
                if state.token_expect_kind(SQLTokenKind::Identifier) {
                    s.columns.push(state.token_current().unwrap().into());
                }
                state.advance();
                if state.token_take_text_sen(",")
                    && state.token_eat_text_sen(")")
                {
                    state.record_trailing_comma();
                    break;
                }
            }
            state.token_take_text_sen(")");
        }

        state.token_take_text_ins("VALUES");

        while state.not_at_end() {
            if state.token_eat_text_sen(";") {
                break;
            }

            if !state.token_take_text_sen("(") {
                state.record_expected_text("(");
                break;
            }
            while state.not_at_end() {
                s.values.push(SQLExpression::new(state));
                if state.token_take_text_sen(")") {
                    break;
                }
                if state.token_take_text_sen(",") {
                    if state.token_eat_text_sen(")") {
                        state.record_trailing_comma();
                        state.advance();
                        break;
                    }
                    continue;
                }
                break;
            }
            if !state.token_take_text_sen(",") {
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
    pub table_name: Token2,
    pub assignments: Vec<SQLAssignment>,
    pub selection: Option<SQLExpression>,
}

impl SQLUpdate {
    pub fn new(state: &mut SQLParserState) -> Self {
        let mut s = Self {
            table_name: Token2::default(),
            assignments: vec![],
            selection: None,
        };

        state.advance();
        state.token_expect_kind(SQLTokenKind::Identifier);
        s.table_name =
            state.token_current().map(Into::into).unwrap_or_default();
        state.advance();
        state.token_take_text_ins("SET");
        while state.not_at_end() {
            if state.token_take_text_ins("WHERE") {
                s.selection = Some(SQLExpression::new(state));
                return s;
            }
            if state.token_eat_text_sen(";") {
                break;
            }
            s.assignments.push(SQLAssignment::new(state));
            if state.token_take_text_sen(",") {
                if state.token_eat_texts_ins(&["WHERE", ";"]) {
                    state.record_trailing_comma();
                }
                continue;
            }
            if state.token_eat_text_ins("WHERE") {
                continue;
            }
            if state.token_eat_text_sen(";") {
                break;
            }
            return s;
        }
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
    pub column: Token2,
    pub value: SQLExpression,
}

impl SQLAssignment {
    pub fn new(state: &mut SQLParserState) -> Self {
        let column = state.token_current().map(Into::into).unwrap_or_default();
        state.advance();
        state.token_take_text_ins("=");
        let value = SQLExpression::new(state);
        SQLAssignment { column, value }
    }
}

#[derive(Debug, Clone)]
pub struct SQLDelete {
    pub table_name: Token2,
    pub selection: Option<SQLExpression>,
}

impl SQLDelete {
    pub fn new(state: &mut SQLParserState) -> Self {
        let mut s = Self { table_name: Token2::default(), selection: None };
        state.advance();
        state.token_take_text_ins("FROM");
        s.table_name =
            state.token_current().map(Into::into).unwrap_or_default();
        state.advance();
        state.token_take_text_ins("WHERE");
        s.selection = Some(SQLExpression::new(state));
        s
    }
}

impl From<SQLDelete> for SQLAst {
    fn from(value: SQLDelete) -> Self {
        Self::Delete(value)
    }
}
use crate::core::parser::pratt::Token2;
