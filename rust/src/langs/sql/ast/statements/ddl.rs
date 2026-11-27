use crate::core::parser::pratt::Token2;
use crate::langs::sql::ast::expression::SQLExpression;
use crate::langs::sql::ast::statements::query::SQLSelect;
use crate::langs::sql::ast::{SQLAst, SQLParserState};
use crate::langs::sql::lexer::SQLTokenKind;

#[derive(Debug, Clone)]
pub struct SQLCreate {
    pub name: Token2,
    pub if_not_exists: bool,
    pub body: SQLCreateBody,
}

impl SQLCreate {
    pub fn new(state: &mut SQLParserState) -> Self {
        state.advance();
        let mut s = Self {
            name: Token2::default(),
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
        } else if state.token_take_text_ins("OR") {
            state.token_take_text_ins("REPLACE");
            state.token_expect_text_ins("VIEW");
            s.if_not_exists = true; // cause we're replacing it anyway
            s.name(state);
            state.token_expect_text_ins("AS");
            s.body = SQLCreateBody::view(state);
        } else if state.token_eat_text_ins("INDEX") {
            s.if_not_exists(state);
            s.name(state);
            s.body = SQLCreateBody::index_(state, false);
        } else if state.token_take_text_ins("UNIQUE") {
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
        if state.token_take_text_ins("NOT") {
            self.if_not_exists = true;
        }

        if state.token_take_text_ins("EXISTS") {}
    }

    fn name(&mut self, state: &mut SQLParserState) {
        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return;
        }

        self.name = state.token_current().unwrap().into();
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
    Index_(Token2, Vec<Token2>, bool), // freezed.dart uses this name for tuple enums?
    Database,
}

impl SQLCreateBody {
    pub fn table(state: &mut SQLParserState) -> Self {
        let mut v = vec![];
        if state.token_take_text_sen("(") {}

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

        if state.token_take_text_ins(")") {}

        Self::Table(v)
    }

    pub fn index_(state: &mut SQLParserState, unique: bool) -> Self {
        let mut target_table = Token2::default();
        let mut columns = vec![];
        if state.token_take_text_ins("ON") {}

        if state.token_expect_kind(SQLTokenKind::Identifier) {
            target_table = state.token_current().unwrap().into();
        }

        state.advance();
        if state.token_take_text_sen("(") {}

        while state.not_at_end() {
            if state.token_expect_kind(SQLTokenKind::Identifier) {
                columns.push(state.token_current().unwrap().into());
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

        if state.token_take_text_ins(")") {}

        Self::Index_(target_table, columns, unique)
    }

    pub fn view(state: &mut SQLParserState) -> Self {
        state.advance();
        if state.token_take_text_ins("select") {}

        Self::View(SQLSelect::new(state))
    }
}

#[derive(Debug, Clone)]
pub struct SQLColumnDefinition {
    pub name: Token2,
    pub r#type: Token2,
    pub constraints: Vec<SQLColumnRestraint>,
}

impl SQLColumnDefinition {
    pub fn new(state: &mut SQLParserState) -> Self {
        let mut s = Self {
            name: Token2::default(),
            r#type: Token2::default(),
            constraints: vec![],
        };

        if state.token_expect_kind(SQLTokenKind::Identifier) {
            s.name = state.token_current().unwrap().into();
        }

        state.advance();
        if state.token_expect_kind(SQLTokenKind::Identifier) {
            s.r#type = parse_type(state);
        }

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
        if state.token_take_text_ins("PRIMARY") {
            state.token_expect_text_ins("KEY");
            Some(Self::PrimaryKey)
        } else if state.token_take_text_ins("FOREIGN") {
            state.token_expect_text_ins("KEY");
            Some(Self::ForeignKey)
        } else if state.token_take_text_ins("UNIQUE") {
            // TODO: I forgot if this is UNIQUE KEY or just UNIQUE
            state.token_eat_text_ins("KEY"); // change this to expect if the former
            Some(Self::Unique)
        } else if state.token_take_text_ins("NOT") {
            state.token_expect_text_ins("NULL");
            Some(Self::NotNull)
        } else if state.token_eat_text_ins("NULL") {
            Some(Self::Nullable)
        } else if state.token_take_text_ins("AUTO") {
            state.token_expect_text_ins("INCREMENT");
            Some(Self::AutoIncrement)
        } else if state.token_eat_text_ins("AUTO_INCREMENT") {
            Some(Self::AutoIncrement)
        } else if state.token_take_text_ins("DEFAULT") {
            Some(Self::Default(SQLExpression::new(state)))
        } else if state.token_take_text_ins("CHECK") {
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
    pub name: Token2,
    pub if_exists: bool,
}

impl SQLDrop {
    pub fn new(state: &mut SQLParserState) -> Self {
        let mut s = Self {
            r#type: SQLDropType::Database,
            name: Token2::default(),
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
        if state.token_take_text_ins("NOT") {
            self.if_exists = false;
        } else {
            self.if_exists = true;
        }

        if state.token_take_text_ins("EXISTS") {}
    }

    fn name(&mut self, state: &mut SQLParserState) {
        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return;
        }

        self.name = state.token_current().unwrap().into();
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
    pub name: Token2,
    pub action: Option<SQLAlterAction>,
}

impl SQLAlter {
    pub fn new(state: &mut SQLParserState) -> Self {
        state.advance();
        let mut s = Self { name: Token2::default(), action: None };

        if !state.token_expect_text_ins("TABLE") {
            return s;
        }
        state.advance();

        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return s;
        }

        s.name = state.token_current().unwrap().into();
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
    AddColumn(Token2, Token2),
    DropColumn(Token2),
    RenameTo(Token2, Token2),
    RenameTable(Token2),
}

impl SQLAlterAction {
    pub fn new(state: &mut SQLParserState) -> Option<Self> {
        if state.token_take_text_ins("ADD") {
            Some(Self::add(state))
        } else if state.token_take_text_ins("DROP") {
            Some(Self::drop(state))
        } else if state.token_take_text_ins("RENAME") {
            Some(Self::rename(state))
        } else {
            state.record_unexpected_token();
            None
        }
    }

    fn add(state: &mut SQLParserState) -> Self {
        let mut name = Token2::default();
        let mut r#type = Token2::default();
        if state.token_take_text_ins("COLUMN") {}

        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return Self::AddColumn(name, r#type);
        }

        name = state.token_current().unwrap().into();
        state.advance();
        r#type = parse_type(state);

        Self::AddColumn(name, r#type)
    }

    fn drop(state: &mut SQLParserState) -> Self {
        let mut name = Token2::default();
        if state.token_take_text_ins("COLUMN") {}

        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return Self::DropColumn(name);
        }

        name = state.token_current().unwrap().into();
        state.advance();

        Self::DropColumn(name)
    }

    fn rename(state: &mut SQLParserState) -> Self {
        if state.token_take_text_ins("TO") {
            if !state.token_expect_kind(SQLTokenKind::Identifier) {
                return Self::RenameTable(Token2::default());
            }
            let new_name = state.token_current().unwrap().into();
            state.advance();
            return Self::RenameTable(new_name);
        }

        let mut name = Token2::default();
        let mut new = Token2::default();
        if state.token_take_text_ins("COLUMN") {}

        state.advance();
        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return Self::RenameTo(name, new);
        }

        name = state.token_current().unwrap().into();
        state.advance();

        if !state.token_eat_text_ins("TO") {
            return Self::RenameTo(name, new);
        }

        state.advance();
        if !state.token_expect_kind(SQLTokenKind::Identifier) {
            return Self::RenameTo(name, new);
        }

        new = state.token_current().unwrap().into();
        state.advance();

        Self::RenameTo(name, new)
    }
}

fn parse_type(state: &mut SQLParserState<'_>) -> Token2 {
    let mut r#type: Token2 =
        state.token_current().map(Into::into).unwrap_or_default();
    state.advance();

    if state.token_take_text_sen("(") {
        r#type.text.push('(');
        while state.not_at_end() && !state.token_eat_text_sen(")") {
            r#type.text.push_str(state.token_text().unwrap_or(""));
            state.advance();
        }
        if state.token_take_text_sen(")") {
            r#type.text.push(')');
        }
    }

    r#type
}
