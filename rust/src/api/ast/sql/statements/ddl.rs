use crate::api::GraceError;
use crate::api::ast::sql::SqlAst;
use crate::api::ast::sql::expression::SqlExpression;
use crate::api::ast::sql::statements::query::SqlSelect;
use crate::api::ast::unexpected_eof;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct SqlCreate {
    pub r#type: SqlCreateType,
    pub name: String,
    pub if_not_exists: bool,
    pub body: SqlCreateBody,
}

impl SqlCreate {
    pub(crate) fn new(
        iter: &mut PrevIter<'_, SQLToken>,
        errors: &mut Vec<GraceError>,
    ) -> Self {
        let mut s = Self::default();
        let _ = iter.next();
        s.name(iter, errors);
        let _ = iter.prev();

        if let Some(token) = iter.next() {
            match token.kind {
                SQLTokenKind::StrictKeyword => {
                    // practically the database is already taken care of
                    // since it's our default constructor
                    let text = token.text.to_string();
                    // eat the name or error since we already did it before matching
                    let _ = iter.next();
                    if text.eq_ignore_ascii_case("table") {}
                }
                _ => {
                    GraceError::syntax_error(errors, token, "unexpected token");
                }
            }
        } else {
            unexpected_eof(iter, errors);
        }

        s
    }

    fn name(
        &mut self,
        iter: &mut PrevIter<'_, SQLToken>,
        errors: &mut Vec<GraceError>,
    ) {
        if let Some(token) = iter.next() {
            if token.kind == SQLTokenKind::Identifier {
                self.name = token.text.into();
            } else {
                GraceError::syntax_error(errors, token, "expected identifier");
            }
        }
    }
}

impl Default for SqlCreate {
    fn default() -> Self {
        Self {
            r#type: SqlCreateType::DATABASE,
            name: "???".into(),
            if_not_exists: false,
            body: SqlCreateBody::Database,
        }
    }
}

impl From<SqlCreate> for SqlAst {
    fn from(value: SqlCreate) -> Self {
        Self::Create(value)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum SqlCreateType {
    TABLE,
    VIEW,
    INDEX,
    DATABASE,
}

#[derive(Debug, Clone)]
pub(crate) enum SqlCreateBody {
    Table(Vec<SqlColumnDefinition>),
    View(SqlSelect),
    Index(String, Vec<String>, bool),
    Database,
}

#[derive(Debug, Clone)]
pub(crate) struct SqlColumnDefinition {
    pub name: String,
    pub r#type: String,
    pub constraints: Vec<SqlColumnRestraint>,
}

#[derive(Debug, Clone)]
pub(crate) enum SqlColumnRestraint {
    PrimaryKey,
    NotNull,
    Nullable,
    Unique,
    Default(SqlExpression),
    Check(SqlExpression),
    AutoIncrement,
}

#[derive(Debug, Clone)]
pub(crate) struct SqlDrop {
    pub r#type: SqlCreateType,
    pub name: String,
    pub if_exists: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct SqlAlter {
    pub name: String,
    pub action: Option<SqlAlterAction>,
}

#[derive(Debug, Clone)]
pub(crate) enum SqlAlterAction {
    AddColumn(String, Option<String>),
    DropColumn(String),
    RenameTo(String),
}
