use std::collections::HashSet;

use crate::core::ast::sql::expression::SQLExpression;
use crate::core::ast::sql::statements::query::SQLSelect;
use crate::core::ast::sql::{SQLAst, SQLParserState};
use crate::core::lexers::sql::{SQLToken, SQLTokenKind};
use crate::core::parser::state::ParserState;

#[derive(Debug, Clone)]
pub struct SQLCreate {
    pub name: String,
    pub if_not_exists: bool,
    pub body: SQLCreateBody,
}

impl SQLCreate {
    pub fn new(state: &mut SQLParserState) -> Self {
        state.advance();
        let mut s = Self {
            name: String::new(),
            if_not_exists: false,
            body: SQLCreateBody::Database,
        };

        if state.token_eat_text_ins("database") {
            s.if_not_exists(state);
            s.name(state);
            s.body = SQLCreateBody::Database; // not really necessary
        } else if state.token_eat_text_ins("table") {
            s.if_not_exists(state);
            s.name(state);
            s.body = SQLCreateBody::table(state);
        } else if state.token_eat_text_ins("view") {
            s.if_not_exists(state);
            s.name(state);
        } else if state.token_eat_text_ins("or") {
            state.advance();
            state.token_expect_text_ins("replace");
            state.advance();
            state.token_expect_text_ins("view");
            s.if_not_exists(state);
            s.name(state);
        } else if state.token_eat_text_ins("index") {
            s.if_not_exists(state);
            s.name(state);
            s.body = SQLCreateBody::index_(state, false);
        } else if state.token_eat_text_ins("unique") {
            state.advance();
            state.token_expect_text_ins("index");
            s.if_not_exists(state);
            s.name(state);
            s.body = SQLCreateBody::index_(state, true);
        } else {
            state.record_unexpected_token();
        }

        s
    }

    fn if_not_exists(&mut self, state: &mut SQLParserState) {
        state.advance();
        if !state.token_eat_text_ins("if") {
            return;
        }

        state.advance();
        if state.token_eat_text_ins("not") {
            state.advance();
            self.if_not_exists = true;
        }

        if state.token_expect_text_ins("exists") {
            state.advance();
        }
    }

    fn name(&mut self, state: &mut SQLParserState) {
        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return;
        }

        self.name = state.token_current().unwrap().text.to_string();
        state.advance();
    }
}

impl From<SQLCreate> for SQLAst {
    fn from(value: SQLCreate) -> Self {
        Self::Create(value)
    }
}

#[derive(Debug, Clone)]
pub enum SQLCreateBody {
    Table(Vec<SQLColumnDefinition>),
    View(SQLSelect),
    Index_(String, Vec<String>, bool), // freezed.dart uses this name for tuple enums?
    Database,
}

impl SQLCreateBody {
    pub fn table(state: &mut SQLParserState) -> Self {
        let mut v = vec![];
        // no need for advance since self.name() already puts it after the name
        if state.token_expect_text_ins("(") {
            state.advance();
        }

        while state.not_at_end() {
            v.push(SQLColumnDefinition::new(state));
            if state.token_eat_texts_ins(&[")", ";"]) {
                break;
            }

            if state.token_expect_text_ins(",") {
                if let Ok(token) = state.get_token_at(1)
                    && token.text == ")"
                {
                    state.advance();
                    state.record_trailing_comma();
                    break;
                }
            }

            state.advance();
        }

        if state.token_expect_text_ins(")") {
            state.advance();
        }

        Self::Table(v)
    }

    pub fn index_(state: &mut SQLParserState, unique: bool) -> Self {
        let mut target_table = String::new();
        let mut columns = vec![];
        // no need for advance since self.name() already puts it after the name
        if state.token_expect_text_ins("on") {
            state.advance();
        }

        if state.token_expect_kind(SQLTokenKind::Identifier) {
            target_table = state.token_current().unwrap().text.to_string();
        }

        state.advance();
        if state.token_expect_text_ins("(") {
            state.advance();
        }

        while state.not_at_end() {
            if state.token_expect_kind(SQLTokenKind::Identifier) {
                columns.push(state.token_current().unwrap().text.to_string());
                state.advance();
            }

            if state.token_eat_texts_ins(&[")", ";"]) {
                break;
            }

            if state.token_expect_text_ins(",") {
                if let Ok(token) = state.get_token_at(1)
                    && token.text == ")"
                {
                    state.advance();
                    state.record_trailing_comma();
                    break;
                }
            }

            state.advance();
        }

        if state.token_expect_text_ins(")") {
            state.advance();
        }

        Self::Index_(target_table, columns, unique)
    }
}

#[derive(Debug, Clone)]
pub struct SQLColumnDefinition {
    pub name: String,
    pub r#type: String,
    pub constraints: Vec<SQLColumnRestraint>,
}

impl SQLColumnDefinition {
    pub fn new(state: &mut SQLParserState) -> Self {
        let mut s = Self {
            name: String::new(),
            r#type: String::new(),
            constraints: vec![],
        };

        if state.token_expect_kind(SQLTokenKind::Identifier) {
            s.name = state.token_current().unwrap().text.to_string();
        }

        state.advance();
        if state.token_expect_kind(SQLTokenKind::Identifier) {
            s.name = state.token_current().unwrap().text.to_string();
        }

        state.advance();
        while state.not_at_end() {
            if state.token_eat_texts_ins(&[",", ")", ";"]) {
                break;
            }

            if let Some(c) = SQLColumnRestraint::new(state) {
                s.constraints.push(c);
            }

            state.advance();
        }

        s
    }
}

#[derive(Debug, Clone)]
pub enum SQLColumnRestraint {
    PrimaryKey,
    ForeignKey,
    Unique,
    NotNull,
    Nullable,
    AutoIncrement,
    Default(SQLExpression), // TODO: SQL Expression Parsing
    Check(SQLExpression),
}

impl SQLColumnRestraint {
    pub fn new(state: &mut SQLParserState) -> Option<Self> {
        if state.token_eat_text_ins("primary") {
            state.advance();
            state.token_expect_text_ins("key");
            Some(Self::PrimaryKey)
        } else if state.token_eat_text_ins("foreign") {
            state.advance();
            state.token_expect_text_ins("key");
            Some(Self::ForeignKey)
        } else if state.token_eat_text_ins("unique") {
            state.advance();
            // TODO: I forgot if this is UNIQUE KEY or just UNIQUE
            state.token_eat_text_ins("key"); // change this to expect if the former
            Some(Self::Unique)
        } else if state.token_eat_text_ins("not") {
            state.advance();
            state.token_expect_text_ins("null");
            Some(Self::NotNull)
        } else if state.token_eat_text_ins("null") {
            Some(Self::Nullable)
        } else if state.token_eat_text_ins("auto") {
            state.advance();
            state.token_expect_text_ins("increment");
            Some(Self::AutoIncrement)
        } else if state.token_eat_text_ins("auto_increment") {
            Some(Self::AutoIncrement)
        } else {
            state.record_unexpected_token();
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct SQLDrop {
    pub r#type: SQLDropType,
    pub name: String,
    pub if_exists: bool,
}

#[derive(Debug, Clone)]
pub enum SQLDropType {
    Table,
    View,
    Index,
    Database,
}

#[derive(Debug, Clone)]
pub struct SQLAlter {
    pub name: String,
    pub action: Option<SQLAlterAction>,
}

#[derive(Debug, Clone)]
pub enum SQLAlterAction {
    AddColumn(String, Option<String>),
    DropColumn(String),
    RenameTo(String),
}
