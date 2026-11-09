use crate::core::ast::sql::expression::SQLExpression;
use crate::core::ast::sql::{SQLAst, SQLParserState};
use crate::core::lexers::sql::{SQLToken, SQLTokenKind};

#[derive(Debug, Clone)]
pub struct SQLSelect {
    pub items: Vec<SQLSelectItem>,
    pub from: Option<String>,
    pub joins: Vec<SQLJoinClause>,
    pub r#where: Option<SQLExpression>,
    pub group_by: Option<SQLGroupByClause>,
    pub having: Option<SQLHavingClause>,
    pub order_by: Option<SQLOrderByClause>,
    pub limit: Option<SQLLimitClause>,
}

impl SQLSelect {
    pub fn new(state: &mut SQLParserState) -> Self {
        let mut s = Self {
            items: vec![],
            from: None,
            joins: vec![],
            r#where: None,
            group_by: None,
            having: None,
            order_by: None,
            limit: None,
        };

        state.advance();
        if state.token_eat_text_ins("distinct") {
            state.advance();
        }

        while state.not_at_end() {
            s.items.push(SQLSelectItem::new(state));
            if state.token_eat_kind(SQLTokenKind::StrictKeyword) {
                break;
            }

            if state.token_expect_text_ins(",") {
                if state.token_peek_at_kind(1)
                    == Some(&SQLTokenKind::StrictKeyword)
                {
                    state.record_trailing_comma();
                    state.advance();
                    break;
                }
            }

            state.advance();
        }

        if state.token_eat_text_ins("from") {
            state.advance();
            state.token_expect_kind(SQLTokenKind::Identifier);
            s.from = state.token_text().map(|t| t.to_string());
            state.advance();
        }

        while state.not_at_end() {
            if state.token_eat_text_ins("where") {
                if s.r#where.is_none() {
                    state.advance();
                    s.r#where = Some(SQLExpression::new(state));
                } else {
                    state.advance();
                    let _ = SQLExpression::new(state);
                    state.record_syntax_error("Where clause was used again");
                }
            }

            /* TODO */
            // the rest...

            if state.token_eat_text_ins(";") {
                break;
            }
        }

        s
    }
}

impl From<SQLSelect> for SQLAst {
    fn from(value: SQLSelect) -> Self {
        Self::Select(value)
    }
}

#[derive(Debug, Clone)]
pub enum SQLSelectItem {
    Asterisk,
    Expr(SQLExpression, Option<SQLExpression>),
}

impl SQLSelectItem {
    pub fn new(state: &mut SQLParserState) -> Self {
        if state.token_eat_text_sen("*") {
            return Self::Asterisk;
        }

        let mut expr = SQLExpression::new(state);
        if state.token_eat_text_ins("as") {
            state.advance();
            return Self::Expr(expr, Some(SQLExpression::new(state)));
        }

        Self::Expr(expr, None)
    }
}

#[derive(Debug, Clone)]
pub struct SQLJoinClause {
    pub join: SQLJoinType,
    pub table: String,
    pub on: Option<SQLExpression>,
}

#[derive(Debug, Clone)]
pub enum SQLJoinType {
    Inner,
    Left,
    Right,
    Full,
}

#[derive(Debug, Clone)]
pub struct SQLGroupByClause {
    pub columns: Vec<SQLExpression>,
}

#[derive(Debug, Clone)]
pub struct SQLHavingClause {
    pub condition: SQLExpression,
}

#[derive(Debug, Clone)]
pub struct SQLOrderByClause {
    pub items: Vec<SQLOrderByItem>,
}

#[derive(Debug, Clone)]
pub struct SQLOrderByItem {
    pub expr: SQLExpression,
    pub direction: SQLOrderDirection,
}

#[derive(Debug, Clone)]
pub enum SQLOrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct SQLLimitClause {
    pub limit: SQLExpression,
    pub offset: Option<SQLExpression>,
}
