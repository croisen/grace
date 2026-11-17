use crate::core::parser::pratt::{
    PrattAssociativity, PrattLanguage, parse_expr,
};
use crate::langs::sql::ast::statements::query::SQLSelect;
use crate::langs::sql::ast::{SQLAst, SQLParserState};
use crate::langs::sql::lexer::{SQLToken, SQLTokenKind};

#[derive(Debug, Clone)]
pub enum SQLExpression {
    Literal(String),
    Identifier(String),
    FunctionCall {
        name: String,
        args: Vec<SQLExpression>,
    },
    UnaryOp {
        op: String,
        expr: Box<SQLExpression>,
        is_prefix: bool,
    },
    BinaryOp {
        op: String,
        left: Box<SQLExpression>,
        right: Box<SQLExpression>,
    },
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
        parse_expr(state, 0).unwrap_or(Self::Identifier("Placeholder".into()))
    }
}

impl<'a> PrattLanguage<'a> for SQLExpression {
    type Root = SQLAst;
    type TokenKind = SQLTokenKind;

    fn parse_atom(state: &mut SQLParserState<'a>) -> Option<Self> {
        if state.token_eat_text_sen("(") {
            state.advance();
            let expr = parse_expr(state, 0);
            state.token_expect_text_sen(")");
            return expr;
        }

        if state.token_eat_kind(SQLTokenKind::Identifier) {
            return Some(Self::Identifier(state.token_text().unwrap().into()));
        }

        if state.token_eat_kinds(&[
            SQLTokenKind::String,
            SQLTokenKind::Integer,
            SQLTokenKind::Float,
        ]) {
            return Some(Self::Literal(state.token_text().unwrap().into()));
        }

        state.record_unexpected_token();
        None
    }

    fn parse_prefix(
        state: &mut SQLParserState<'a>,
        op: String,
        rhs: Self,
    ) -> Option<Self> {
        Some(Self::UnaryOp { op, expr: rhs.into(), is_prefix: true })
    }

    fn parse_infix(
        state: &mut SQLParserState<'a>,
        op: String,
        lhs: Self,
        rhs: Self,
    ) -> Option<Self> {
        Some(Self::BinaryOp { op, left: lhs.into(), right: rhs.into() })
    }

    fn parse_postfix(
        state: &mut SQLParserState<'a>,
        op: String,
        lhs: Self,
    ) -> Option<Self> {
        // not like this could happen but we're gonna be precise here, but our postfix operator is only
        // functions right now
        if op != "(" {
            state.record_unexpected_token();
            return None;
        }

        let Self::Identifier(name) = &lhs else {
            state.seek_offset(-1);
            state.record_custom("Function callee should be an identifier");
            state.seek_offset(1);
            return None;
        };

        let mut args = vec![];
        loop {
            args.push(parse_expr(state, 0)?);
            if state.token_eat_text_ins(")") {
                break;
            }

            if state.token_expect_text_ins(",") {
                if state.token_peek_at_text(1) == Some(")") {
                    state.record_trailing_comma();
                    state.advance();
                    break;
                }
            }

            state.advance();
        }

        Some(Self::FunctionCall { name: name.clone(), args })
    }

    fn prefix_binding(token: &SQLToken<'a>) -> Option<PrattAssociativity> {
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
            "*" => Some(PrattAssociativity::Left(7)),
            "/" => Some(PrattAssociativity::Left(7)),
            "%" => Some(PrattAssociativity::Left(7)),

            // Add
            "+" => Some(PrattAssociativity::Left(6)),
            "-" => Some(PrattAssociativity::Left(6)),

            // Comp
            "<" => Some(PrattAssociativity::None(5)),
            ">" => Some(PrattAssociativity::None(5)),
            "<=" => Some(PrattAssociativity::None(5)),
            ">=" => Some(PrattAssociativity::None(5)),

            // Equal
            "==" => Some(PrattAssociativity::None(4)),
            "!=" => Some(PrattAssociativity::None(4)), // SQL not equal
            "<>" => Some(PrattAssociativity::None(4)), // SQL not equal

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
            // "[" => Some(PrattAssociativity::Left(10)),
            _ => None,
        }
    }
}
