use crate::core::parser::pratt::Token2;
use crate::langs::sql::ast::expression::SQLExpression;
use crate::langs::sql::ast::{SQLAst, SQLParserState};
use crate::langs::sql::lexer::SQLTokenKind;

#[derive(Debug, Clone)]
pub struct SQLSelect {
    pub items: Vec<SQLSelectItem>,
    pub from: Option<Token2>,
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
        state.token_take_text_ins("distinct");

        while state.not_at_end() {
            if state.token_eat_text_sen(";") {
                break;
            }
            if state.token_take_text_ins("FROM") {
                state.token_expect_kind(SQLTokenKind::Identifier);
                s.from = state.token_current().map(Into::into);
                state.advance();
                break;
            }

            if state.token_eat_texts_ins(&[
                "WHERE", "GROUP", "HAVING", "ORDER", "LIMIT",
            ]) {
                break;
            }

            s.items.push(SQLSelectItem::new(state));
            if state.token_eat_text_sen(";") {
                break;
            }
            if state.token_take_text_sen(",") {
                if state.token_eat_texts_ins(&[
                    "FROM", "WHERE", "GROUP", "HAVING", "ORDER", "LIMIT",
                ]) {
                    state.record_trailing_comma();
                    break;
                }
                continue;
            }

            if state.token_eat_texts_ins(&[
                "FROM", "WHERE", "GROUP", "HAVING", "ORDER", "LIMIT",
            ]) {
                continue;
            }

            if state.not_at_end() {
                state.record_unexpected_token();
                state.advance();
            }
        }

        if s.from.is_none() && state.token_take_text_ins("FROM") {
            state.token_expect_kind(SQLTokenKind::Identifier);
            s.from = state.token_current().map(Into::into);
            state.advance();
        }

        while state.not_at_end() {
            let join = if state.token_take_text_ins("LEFT") {
                state.token_take_text_ins("OUTER");
                state.token_expect_take_text_ins("JOIN");
                Some(SQLJoinType::Left)
            } else if state.token_take_text_ins("RIGHT") {
                state.token_take_text_ins("OUTER");
                state.token_expect_take_text_ins("JOIN");
                Some(SQLJoinType::Right)
            } else if state.token_take_text_ins("FULL") {
                state.token_take_text_ins("OUTER");
                state.token_expect_take_text_ins("JOIN");
                Some(SQLJoinType::Full)
            } else if state.token_take_text_ins("INNER") {
                state.token_expect_take_text_ins("JOIN");
                Some(SQLJoinType::Inner)
            } else if state.token_take_text_ins("JOIN") {
                Some(SQLJoinType::Inner)
            } else {
                None
            };

            if let Some(join) = join {
                state.token_expect_kind(SQLTokenKind::Identifier);
                let table =
                    state.token_current().map(Into::into).unwrap_or_default();
                state.advance();
                let on = if state.token_take_text_ins("ON") {
                    Some(SQLExpression::new(state))
                } else {
                    None
                };
                s.joins.push(SQLJoinClause { join, table, on });
            } else if state.token_eat_text_ins("WHERE") {
                if s.r#where.is_none() {
                    state.advance();
                    s.r#where = Some(SQLExpression::new(state));
                } else {
                    state.advance();
                    let _ = SQLExpression::new(state);
                    state.record_syntax_error("Where clause was used again");
                }
            } else if state.token_take_text_ins("GROUP") {
                state.token_take_text_ins("BY");
                let mut columns = vec![SQLExpression::new(state)];
                while state.token_take_text_sen(",") {
                    columns.push(SQLExpression::new(state));
                }
                s.group_by = Some(SQLGroupByClause { columns });
            } else if state.token_take_text_ins("HAVING") {
                s.having = Some(SQLHavingClause {
                    condition: SQLExpression::new(state),
                });
            } else if state.token_take_text_ins("ORDER") {
                state.token_take_text_ins("BY");
                let mut items = vec![];
                loop {
                    let expr = SQLExpression::new(state);
                    let direction =
                        if state.token_current().is_some_and(|token| {
                            token.text.eq_ignore_ascii_case("DESC")
                        }) {
                            state.advance();
                            SQLOrderDirection::Desc
                        } else {
                            if state.token_current().is_some_and(|token| {
                                token.text.eq_ignore_ascii_case("ASC")
                            }) {
                                state.advance();
                            }
                            SQLOrderDirection::Asc
                        };
                    items.push(SQLOrderByItem { expr, direction });
                    if !state.token_eat_text_sen(",") {
                        break;
                    }
                    state.advance();
                }
                s.order_by = Some(SQLOrderByClause { items });
            } else if state.token_take_text_ins("LIMIT") {
                let limit = SQLExpression::new(state);
                let offset = if state.token_take_text_ins("OFFSET") {
                    Some(SQLExpression::new(state))
                } else {
                    None
                };
                s.limit = Some(SQLLimitClause { limit, offset });
            } else {
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
        if state.token_take_text_sen("*") {
            return Self::Asterisk;
        }

        let expr = SQLExpression::new(state);
        if state.token_take_text_ins("AS") {
            return Self::Expr(expr, Some(SQLExpression::new(state)));
        }

        Self::Expr(expr, None)
    }
}

#[derive(Debug, Clone)]
pub struct SQLJoinClause {
    pub join: SQLJoinType,
    pub table: Token2,
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
