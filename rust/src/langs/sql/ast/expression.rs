use crate::core::parser::pratt::{
    PrattAssociativity, PrattLanguage, Token2, parse_expr,
};
use crate::langs::sql::ast::statements::query::SQLSelect;
use crate::langs::sql::ast::{SQLAst, SQLParserState};
use crate::langs::sql::lexer::{SQLToken, SQLTokenKind};

#[derive(Debug, Clone)]
pub enum SQLExpression {
    Literal(Token2),
    Identifier(Token2),
    FunctionCall {
        name: Token2,
        args: Vec<SQLExpression>,
    },
    UnaryOp {
        op: Token2,
        expr: Box<SQLExpression>,
        is_prefix: bool,
    },
    BinaryOp {
        op: Token2,
        left: Box<SQLExpression>,
        right: Box<SQLExpression>,
    },
    // not sure about this though. It's in the spec but I haven't encountered it
    Vector(Vec<SQLExpression>),
    InList {
        expr: Box<SQLExpression>,
        list: Vec<SQLExpression>,
        negated: bool,
    },
    Between {
        expr: Box<SQLExpression>,
        low: Box<SQLExpression>,
        high: Box<SQLExpression>,
        negated: bool,
    },
    SubQuery(Box<SQLSelect>),
}

impl SQLExpression {
    pub fn new<'a>(state: &mut SQLParserState<'a>) -> Self {
        parse_expr::<Self>(state, 0)
            .unwrap_or(Self::Identifier(Token2::default()))
    }

    fn parse_between<'a>(
        state: &mut SQLParserState<'a>,
        negated: bool,
        min_bp: isize,
        lhs: Self,
    ) -> Option<Self> {
        let Some(low) = parse_expr::<Self>(state, min_bp) else {
            state.record_syntax_error(
                "Lower bound of BETWEEN statement can't be parsed properly",
            );

            return None;
        };

        if state.token_expect_text_ins("and") {
            state.advance();
        }

        let Some(high) = parse_expr::<Self>(state, min_bp) else {
            state.record_syntax_error(
                "Higher bound of BETWEEN statement can't be parsed properly",
            );

            return None;
        };

        Some(Self::Between {
            expr: lhs.into(),
            low: low.into(),
            high: high.into(),
            negated,
        })
    }

    fn parse_inlist<'a>(
        state: &mut SQLParserState<'a>,
        negated: bool,
        lhs: Self,
    ) -> Option<Self> {
        let mut list = vec![];
        while state.not_at_end() {
            let Some(expr) = parse_expr::<Self>(state, 0) else {
                state.record_syntax_error(
                    "Failed to parse expression for in list statement",
                );

                return None;
            };

            list.push(expr);
            if state.token_eat_kind(SQLTokenKind::StrictKeyword) {
                break;
            }

            if state.token_expect_text_sen(",") {
                state.advance();
                if state.token_eat_kind(SQLTokenKind::StrictKeyword) {
                    state.record_trailing_comma();
                    break;
                }

                state.seek_offset(-1);
            }

            state.advance();
        }

        Some(Self::InList { expr: lhs.into(), list, negated })
    }

    fn parse_negated<'a>(
        state: &mut SQLParserState<'a>,
        min_bp: isize,
        op: Token2,
        lhs: Self,
    ) -> Option<Self> {
        if state.token_eat_text_ins("in") {
            Self::parse_inlist(state, true, lhs)
        } else if state.token_eat_text_ins("between") {
            let binding =
                Self::infix_binding(state.token_current().unwrap()).unwrap();

            Self::parse_between(state, true, binding.left_power(), lhs)
        } else {
            Some(Self::BinaryOp {
                op,
                left: lhs.into(),
                right: parse_expr::<Self>(state, min_bp)?.into(),
            })
        }
    }
}

impl<'a> PrattLanguage<'a> for SQLExpression {
    type Root = SQLAst;
    type TokenKind = SQLTokenKind;

    fn parse_atom(state: &mut SQLParserState<'a>) -> Option<Self> {
        if state.token_eat_text_sen("(") {
            state.advance();
            let expr = parse_expr::<Self>(state, 0);
            state.token_expect_take_text_sen(")");
            return expr;
        }

        if state.token_eat_text_sen("*") {
            return Some(Self::Identifier(
                state.token_current().unwrap().into(),
            ));
        }

        if state.token_eat_kind(SQLTokenKind::Identifier) {
            return Some(Self::Identifier(
                state.token_current().unwrap().into(),
            ));
        }

        if state.token_eat_kinds(&[
            SQLTokenKind::String,
            SQLTokenKind::Integer,
            SQLTokenKind::Float,
        ]) {
            return Some(Self::Literal(state.token_current().unwrap().into()));
        }

        if state.token_eat_text_ins("select") {
            return Some(Self::SubQuery(SQLSelect::new(state).into()));
        }

        state.record_unexpected_token();
        None
    }

    fn parse_prefix(
        state: &mut SQLParserState<'a>,
        min_bp: isize,
        op: Token2,
    ) -> Option<Self> {
        Some(Self::UnaryOp {
            op,
            expr: parse_expr::<Self>(state, min_bp)?.into(),
            is_prefix: true,
        })
    }

    fn parse_infix(
        state: &mut SQLParserState<'a>,
        min_bp: isize,
        op: Token2,
        lhs: Self,
    ) -> Option<Self> {
        if op.eq_ignore_ascii_case("not") {
            state.advance();
            Self::parse_negated(state, min_bp, op, lhs)
        } else if op.eq_ignore_ascii_case("between") {
            Self::parse_between(state, false, min_bp, lhs)
        } else if op.eq_ignore_ascii_case("in") {
            Self::parse_inlist(state, false, lhs)
        } else {
            Some(Self::BinaryOp {
                op,
                left: lhs.into(),
                right: parse_expr::<Self>(state, min_bp)?.into(),
            })
        }
    }

    fn parse_postfix(
        state: &mut SQLParserState<'a>,
        op: Token2,
        lhs: Self,
    ) -> Option<Self> {
        // not like this could happen but we're gonna be precise here, but our postfix operator is only
        // functions right now
        if op.text != "(" {
            state.record_unexpected_token();
            return None;
        }

        let Self::Identifier(name) = lhs else {
            state.seek_offset(-1);
            state.record_custom("Function callee should be an identifier");
            state.seek_offset(1);
            return None;
        };

        let mut args = vec![];
        loop {
            if state.token_take_text_ins(")") {
                break;
            }

            args.push(parse_expr(state, 0)?);
            if state.token_take_text_ins(")") {
                break;
            }

            if state.token_take_text_ins(",") {
                if state.token_eat_text_ins(")") {
                    state.record_trailing_comma();
                    state.token_take_text_ins(")");
                    break;
                }
                continue;
            }

            state.record_expected_text(", or )");
            break;
        }

        Some(Self::FunctionCall { name, args })
    }

    fn prefix_binding(token: &SQLToken<'a>) -> Option<PrattAssociativity> {
        if token.text.eq_ignore_ascii_case("not") {
            return Some(PrattAssociativity::Right(10));
        }

        match token.text {
            "+" => Some(PrattAssociativity::Right(9)),
            "-" => Some(PrattAssociativity::Right(9)),
            _ => None,
        }
    }

    fn infix_binding(token: &SQLToken<'a>) -> Option<PrattAssociativity> {
        if token.text.eq_ignore_ascii_case("between") {
            return Some(PrattAssociativity::None(8));
        }

        if token.text.eq_ignore_ascii_case("not") {
            return Some(PrattAssociativity::Left(8));
        }

        if token.text.eq_ignore_ascii_case("in") {
            return Some(PrattAssociativity::None(8));
        }

        if token.text.eq_ignore_ascii_case("and") {
            return Some(PrattAssociativity::Left(3));
        }

        if token.text.eq_ignore_ascii_case("or") {
            return Some(PrattAssociativity::Left(2));
        }

        match token.text {
            // Multiply
            "*" | "/" | "%" => Some(PrattAssociativity::Left(7)),

            // Add
            "+" | "-" => Some(PrattAssociativity::Left(6)),

            // Comp
            "<" | ">" | "<=" | ">=" => Some(PrattAssociativity::None(5)),

            // Equal
            "==" | "!=" | "<>" => Some(PrattAssociativity::None(4)),

            // Logical
            "&&" => Some(PrattAssociativity::Left(3)),
            "||" => Some(PrattAssociativity::Left(2)),

            // Assignment
            "=" => Some(PrattAssociativity::Right(1)),
            "+=" => Some(PrattAssociativity::Right(1)),
            "-=" => Some(PrattAssociativity::Right(1)),
            "*=" => Some(PrattAssociativity::Right(1)),
            "%=" => Some(PrattAssociativity::Right(1)),
            "/=" => Some(PrattAssociativity::Right(1)),
            _ => None,
        }
    }

    fn postfix_binding(token: &SQLToken<'a>) -> Option<PrattAssociativity> {
        match token.text {
            // function call
            "(" => Some(PrattAssociativity::Left(10)),
            // array access, though this is different in SQL impls
            // "[" | "->" => Some(PrattAssociativity::Left(10)),
            _ => None,
        }
    }
}
