use std::collections::HashSet;

use crate::api::GraceError;
use crate::api::ast::sql::SQLAst;
use crate::api::ast::sql::expression::SQLExpression;
use crate::api::ast::sql::statements::query::SQLSelect;
use crate::api::ast::unexpected_eof;
use crate::api::lexers::sql::{SQLToken, SQLTokenKind};
use crate::utils::macros::iter_unwrap;
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct SQLCreate {
    pub name: String,
    pub if_not_exists: bool,
    pub body: SQLCreateBody,
}

impl SQLCreate {
    pub(crate) fn new(
        iter: &mut PrevIter<'_, SQLToken>,
        errors: &mut Vec<GraceError>,
    ) -> Self {
        let mut s = Self::default();

        if let Some(token) = iter.next() {
            let text = token.text.to_string();
            match token.kind {
                SQLTokenKind::StrictKeyword => {
                    let token2 = iter_unwrap!(peek, iter, errors, s);
                    if token2.text.eq_ignore_ascii_case("if") {
                        let _ = iter.next();
                        let mut token3 = iter_unwrap!(peek, iter, errors, s);
                        if token3.text.eq_ignore_ascii_case("not") {
                            s.if_not_exists = true;
                            let _ = iter.next();
                            token3 = iter_unwrap!(peek, iter, errors, s);
                        }

                        if !token3.text.eq_ignore_ascii_case("exists") {
                            GraceError::syntax_error(
                                errors,
                                token3,
                                "expected `EXISTS`",
                            );
                        }

                        let _ = iter.next();
                    }

                    s.name(iter, errors);
                    if text.eq_ignore_ascii_case("database") {
                        s.database(iter, errors);
                    } else if text.eq_ignore_ascii_case("table") {
                        s.table(iter, errors);
                    }
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

    fn database(
        &mut self,
        iter: &mut PrevIter<'_, SQLToken>,
        errors: &mut Vec<GraceError>,
    ) {
        if let Some(token) = iter.peek()
            && token.text == ";"
        {
            return;
        }

        let token = iter_unwrap!(iter, errors);
        if token.text != ";" {
            GraceError::syntax_error(errors, token, "expected `;`");
            return;
        }

        // let the main loop eat the semicolon
        let _ = iter.prev();
    }

    fn table(
        &mut self,
        iter: &mut PrevIter<'_, SQLToken>,
        errors: &mut Vec<GraceError>,
    ) {
        self.body = SQLCreateBody::new_table(iter, errors);
    }
}

impl Default for SQLCreate {
    fn default() -> Self {
        Self {
            name: "???".into(),
            if_not_exists: false,
            body: SQLCreateBody::Database,
        }
    }
}

impl From<SQLCreate> for SQLAst {
    fn from(value: SQLCreate) -> Self {
        Self::Create(value)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum SQLCreateBody {
    Table(Vec<SQLColumnDefinition>),
    View(SQLSelect),
    Index_(String, Vec<String>, bool), // freezed.dart uses this name for tuple enums?
    Database,
}

impl SQLCreateBody {
    pub(crate) fn new_table(
        iter: &mut PrevIter<'_, SQLToken>,
        errors: &mut Vec<GraceError>,
    ) -> Self {
        let mut c = vec![];
        let token = iter_unwrap!(peek, iter, errors, Self::Table(c));
        if token.text != "(" {
            GraceError::syntax_error(errors, token, "expected `(`");
        }

        let _ = iter.next();
        while let Some(token) = iter.peek() {
            match token.kind {
                SQLTokenKind::Identifier => {
                    c.push(SQLColumnDefinition::new(iter, errors))
                }
                SQLTokenKind::Delimiters | SQLTokenKind::Pairs => break,
                _ => {}
            }
        }

        let token = iter_unwrap!(peek, iter, errors, Self::Table(c));
        let text = token.text.to_string();
        if text != ")" && text != ";" {
            GraceError::syntax_error(errors, token, "expected `)`");
        }

        if text != ";" {
            let _ = iter.next();
        }

        Self::Table(c)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SQLColumnDefinition {
    pub name: String,
    pub r#type: String,
    pub constraints: Vec<SQLColumnRestraint>,
}

impl SQLColumnDefinition {
    pub(crate) fn new(
        iter: &mut PrevIter<'_, SQLToken>,
        errors: &mut Vec<GraceError>,
    ) -> Self {
        let mut c = Self {
            name: String::new(),
            r#type: String::new(),
            constraints: vec![],
        };

        let token = iter_unwrap!(iter, errors, c);
        if token.kind == SQLTokenKind::Identifier {
            c.name = token.text.into();
        } else {
            c.name = "???".into();
            GraceError::syntax_error(errors, token, "expected identifier");
            return c;
        }

        let token = iter_unwrap!(iter, errors, c);
        if token.kind == SQLTokenKind::Identifier {
            c.r#type = token.text.into();
        } else {
            c.r#type = "???".into();
            GraceError::syntax_error(errors, token, "expected identifier");
            return c;
        }

        while let Some(token) = iter.next() {
            match token.kind {
                SQLTokenKind::StrictKeyword | SQLTokenKind::Identifier => {
                    if let Some(constraint) =
                        SQLColumnRestraint::new(iter, errors)
                    {
                        c.constraints.push(constraint);
                    }
                }
                SQLTokenKind::Pairs if token.text == ")" => break,
                SQLTokenKind::Delimiters => break,
                _ => {
                    GraceError::syntax_error(errors, token, "unexpected token");
                }
            }
        }

        // TODO: Dedup constraints

        c
    }
}

#[derive(Debug, Clone)]
pub(crate) enum SQLColumnRestraint {
    PrimaryKey,
    ForeignKey,
    NotNull,
    Nullable,
    Unique,
    Default(SQLExpression),
    Check(SQLExpression),
    AutoIncrement,
}

impl SQLColumnRestraint {
    pub(crate) fn new(
        iter: &mut PrevIter<'_, SQLToken>,
        errors: &mut Vec<GraceError>,
    ) -> Option<Self> {
        let token = iter_unwrap!(at(iter.pos()), iter, errors, None);
        if token.text.eq_ignore_ascii_case("primary") {
            let token2 = iter_unwrap!(iter, errors, Some(Self::PrimaryKey));
            if !token2.text.eq_ignore_ascii_case("key") {
                GraceError::syntax_error(errors, token2, "expected `KEY`");
            }

            Some(Self::PrimaryKey)
        } else if token.text.eq_ignore_ascii_case("foreign") {
            let token2 = iter_unwrap!(iter, errors, Some(Self::ForeignKey));
            if !token2.text.eq_ignore_ascii_case("key") {
                GraceError::syntax_error(errors, token2, "expected `KEY`");
            }

            Some(Self::ForeignKey)
        } else if token.text.eq_ignore_ascii_case("null") {
            Some(Self::Nullable)
        } else if token.text.eq_ignore_ascii_case("not") {
            let token2 = iter_unwrap!(iter, errors, Some(Self::NotNull));
            if !token2.text.eq_ignore_ascii_case("null") {
                GraceError::syntax_error(errors, token2, "expected `NULL`");
            }

            Some(Self::NotNull)
        } else if token.text.eq_ignore_ascii_case("unique") {
            let token2 = iter_unwrap!(iter, errors, Some(Self::Unique));
            if !token2.text.eq_ignore_ascii_case("key") {
                GraceError::syntax_error(errors, token2, "expected `KEY`");
            }

            Some(Self::Unique)
        } else if token.text.eq_ignore_ascii_case("auto_increment") {
            Some(Self::AutoIncrement)
        } else if token.text.eq_ignore_ascii_case("auto") {
            let token2 = iter_unwrap!(iter, errors, Some(Self::AutoIncrement));
            if !token2.text.eq_ignore_ascii_case("increment") {
                GraceError::syntax_error(
                    errors,
                    token2,
                    "expected `INCREMENT`",
                );
            }

            Some(Self::AutoIncrement)
        } else {
            GraceError::syntax_error(errors, token, "--unexpected token");
            None
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SQLDrop {
    pub r#type: SQLDropType,
    pub name: String,
    pub if_exists: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum SQLDropType {
    Table,
    View,
    Index,
    Database,
}

#[derive(Debug, Clone)]
pub(crate) struct SQLAlter {
    pub name: String,
    pub action: Option<SQLAlterAction>,
}

#[derive(Debug, Clone)]
pub(crate) enum SQLAlterAction {
    AddColumn(String, Option<String>),
    DropColumn(String),
    RenameTo(String),
}
