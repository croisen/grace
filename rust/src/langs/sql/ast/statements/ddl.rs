use crate::langs::sql::ast::expression::SQLExpression;
use crate::langs::sql::ast::statements::query::SQLSelect;
use crate::langs::sql::ast::{SQLAst, SQLParserState};
use crate::langs::sql::lexer::SQLTokenKind;

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

        if state.token_eat_text_ins("DATABASE") {
            s.if_not_exists(state);
            s.name(state);
            s.body = SQLCreateBody::Database; // not really necessary
        } else if state.token_eat_text_ins("TABLE") {
            s.if_not_exists(state);
            s.name(state);
            s.body = SQLCreateBody::table(state);
        } else if state.token_eat_text_ins("VIEW") {
            s.if_not_exists(state);
            s.name(state);
            state.token_expect_text_ins("AS");
            s.body = SQLCreateBody::view(state);
        } else if state.token_eat_text_ins("OR") {
            state.advance();
            state.token_expect_text_ins("REPLACE");
            state.advance();
            state.token_expect_text_ins("VIEW");
            s.if_not_exists = true; // cause we're replacing it anyway
            s.name(state);
            state.token_expect_text_ins("AS");
            s.body = SQLCreateBody::view(state);
        } else if state.token_eat_text_ins("INDEX") {
            s.if_not_exists(state);
            s.name(state);
            s.body = SQLCreateBody::index_(state, false);
        } else if state.token_eat_text_ins("UNIQUE") {
            state.advance();
            state.token_expect_text_ins("INDEX");
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
        if !state.token_eat_text_ins("IF") {
            return;
        }

        state.advance();
        if state.token_eat_text_ins("NOT") {
            state.advance();
            self.if_not_exists = true;
        }

        if state.token_expect_text_ins("EXISTS") {
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
        if state.token_expect_text_sen("(") {
            state.advance();
        }

        while state.not_at_end() {
            v.push(SQLColumnDefinition::new(state));
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

        if state.token_expect_text_ins(")") {
            state.advance();
        }

        Self::Table(v)
    }

    pub fn index_(state: &mut SQLParserState, unique: bool) -> Self {
        let mut target_table = String::new();
        let mut columns = vec![];
        if state.token_expect_text_ins("ON") {
            state.advance();
        }

        if state.token_expect_kind(SQLTokenKind::Identifier) {
            target_table = state.token_current().unwrap().text.to_string();
        }

        state.advance();
        if state.token_expect_text_sen("(") {
            state.advance();
        }

        while state.not_at_end() {
            if state.token_expect_kind(SQLTokenKind::Identifier) {
                columns.push(state.token_current().unwrap().text.to_string());
                state.advance();
            }

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

        if state.token_expect_text_ins(")") {
            state.advance();
        }

        Self::Index_(target_table, columns, unique)
    }

    pub fn view(state: &mut SQLParserState) -> Self {
        state.advance();
        if state.token_expect_text_ins("select") {
            state.advance();
        }

        Self::View(SQLSelect::new(state))
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
            s.name = state.token_text().unwrap().to_string();
        }

        state.advance();
        if state.token_expect_kind(SQLTokenKind::Identifier) {
            s.r#type = state.token_text().unwrap().to_string();
        }

        state.advance();
        while state.not_at_end() {
            if state.token_eat_texts_sen(&[",", ")", ";"]) {
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
    Default(SQLExpression),
    Check(SQLExpression),
}

impl SQLColumnRestraint {
    pub fn new(state: &mut SQLParserState) -> Option<Self> {
        if state.token_eat_text_ins("PRIMARY") {
            state.advance();
            state.token_expect_text_ins("KEY");
            Some(Self::PrimaryKey)
        } else if state.token_eat_text_ins("FOREIGN") {
            state.advance();
            state.token_expect_text_ins("KEY");
            Some(Self::ForeignKey)
        } else if state.token_eat_text_ins("UNIQUE") {
            state.advance();
            // TODO: I forgot if this is UNIQUE KEY or just UNIQUE
            state.token_eat_text_ins("KEY"); // change this to expect if the former
            Some(Self::Unique)
        } else if state.token_eat_text_ins("NOT") {
            state.advance();
            state.token_expect_text_ins("NULL");
            Some(Self::NotNull)
        } else if state.token_eat_text_ins("NULL") {
            Some(Self::Nullable)
        } else if state.token_eat_text_ins("AUTO") {
            state.advance();
            state.token_expect_text_ins("INCREMENT");
            Some(Self::AutoIncrement)
        } else if state.token_eat_text_ins("AUTO_INCREMENT") {
            Some(Self::AutoIncrement)
        } else if state.token_eat_text_ins("DEFAULT") {
            state.advance();
            Some(Self::Default(SQLExpression::new(state)))
        } else if state.token_eat_text_ins("CHECK") {
            state.advance();
            Some(Self::Check(SQLExpression::new(state)))
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

impl SQLDrop {
    pub fn new(state: &mut SQLParserState) -> Self {
        let mut s = Self {
            r#type: SQLDropType::Database,
            name: String::new(),
            if_exists: false,
        };

        state.advance();

        if state.token_eat_text_ins("DATABASE") {
            s.r#type = SQLDropType::Database; // not really necessary
        } else if state.token_eat_text_ins("TABLE") {
            s.r#type = SQLDropType::Table;
        } else if state.token_eat_text_ins("VIEW") {
            s.r#type = SQLDropType::View;
        } else if state.token_eat_text_ins("INDEX") {
            s.r#type = SQLDropType::Index;
        } else {
            state.record_unexpected_token();
        }

        s.if_exists(state);
        s.name(state);

        s
    }

    fn if_exists(&mut self, state: &mut SQLParserState) {
        state.advance();
        if !state.token_eat_text_ins("IF") {
            return;
        }

        state.advance();
        if state.token_eat_text_ins("NOT") {
            state.advance();
            self.if_exists = false;
        } else {
            self.if_exists = true;
        }

        if state.token_expect_text_ins("EXISTS") {
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

impl From<SQLDrop> for SQLAst {
    fn from(value: SQLDrop) -> Self {
        Self::Drop(value)
    }
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

impl SQLAlter {
    pub fn new(state: &mut SQLParserState) -> Self {
        state.advance();
        let mut s = Self { name: String::new(), action: None };

        if !state.token_expect_text_ins("TABLE") {
            return s;
        }

        state.advance();
        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return s;
        }

        s.name = state.token_text().unwrap().into();
        state.advance();
        s.action = SQLAlterAction::new(state);
        s
    }
}

impl From<SQLAlter> for SQLAst {
    fn from(value: SQLAlter) -> Self {
        Self::Alter(value)
    }
}

#[derive(Debug, Clone)]
pub enum SQLAlterAction {
    AddColumn(String, String),
    DropColumn(String),
    RenameTo(String, String),
}

impl SQLAlterAction {
    pub fn new(state: &mut SQLParserState) -> Option<Self> {
        if state.token_eat_text_ins("ADD") {
            state.advance();
            Some(Self::add(state))
        } else if state.token_eat_text_ins("DROP") {
            state.advance();
            Some(Self::drop(state))
        } else if state.token_eat_text_ins("RENAME") {
            state.advance();
            Some(Self::rename(state))
        } else {
            state.record_unexpected_token();
            None
        }
    }

    fn add(state: &mut SQLParserState) -> Self {
        let mut name = String::new();
        let mut r#type = String::new();
        if state.token_expect_text_ins("COLUMN") {
            return Self::AddColumn(name, r#type);
        }

        state.advance();
        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return Self::AddColumn(name, r#type);
        }

        name = state.token_text().unwrap().into();
        state.advance();
        r#type = state.token_text().unwrap().into();
        state.advance();

        Self::AddColumn(name, r#type)
    }

    fn drop(state: &mut SQLParserState) -> Self {
        let mut name = String::new();
        if state.token_expect_text_ins("COLUMN") {
            return Self::DropColumn(name);
        }

        state.advance();
        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return Self::DropColumn(name);
        }

        name = state.token_text().unwrap().into();
        state.advance();

        Self::DropColumn(name)
    }

    fn rename(state: &mut SQLParserState) -> Self {
        let mut name = String::new();
        let mut new = String::new();
        if state.token_expect_text_ins("COLUMN") {
            return Self::RenameTo(name, new);
        }

        state.advance();
        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return Self::RenameTo(name, new);
        }

        name = state.token_text().unwrap().into();
        state.advance();

        if !state.token_eat_text_ins("TO") {
            return Self::RenameTo(name, new);
        }

        state.advance();
        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return Self::RenameTo(name, new);
        }

        new = state.token_text().unwrap().into();
        state.advance();

        Self::RenameTo(name, new)
    }
}
