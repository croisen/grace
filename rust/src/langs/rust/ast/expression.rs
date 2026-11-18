use crate::core::parser::pratt::{
    PrattAssociativity, PrattLanguage, Token2, parse_expr,
};
use crate::langs::rust::ast::pattern::RustPattern;
use crate::langs::rust::ast::statement::RustStatement;
use crate::langs::rust::ast::{RustAst, RustParserState};
use crate::langs::rust::lexer::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub enum RustExpression {
    Identifier(Token2),
    Literal(Token2),
    Path(Vec<Token2>),
    UnaryMutable {
        op: Token2,
        r#mut: bool,
        expr: Box<RustExpression>,
    },
    UnaryOp {
        op: Token2,
        expr: Box<RustExpression>,
    },
    BinaryOp {
        op: Token2,
        left: Box<RustExpression>,
        right: Box<RustExpression>,
    },
    FunctionCall {
        callee: Box<RustExpression>,
        args: Vec<RustExpression>,
    },
    FieldAccess {
        receiver: Box<RustExpression>,
        field: Token2,
    },
    Index {
        receiver: Box<RustExpression>,
        index: Box<RustExpression>,
    },
    Parenthesized(Box<RustExpression>), // we don't use this but rust differentiates it
    Block(Vec<RustBlock>),
    If {
        cond: Box<RustExpression>,
        r#then: RustBlock,
        r#else: Option<RustBlock>,
    },
    While {
        cond: Box<RustExpression>,
        body: RustBlock,
    },
    For {
        var: Token2,
        iter: Box<RustExpression>,
        body: RustBlock,
    },
    Loop(RustBlock),
    Match(Box<RustExpression>, Vec<RustMatchArm>),
    Tuple(Vec<RustExpression>),
    Array(Vec<RustExpression>),
    Struct(Vec<Token2>, Vec<RustFieldInit>),
}

impl RustExpression {
    pub fn new<'a>(state: &mut RustParserState<'a>) -> Self {
        parse_expr::<Self>(state, 0)
            .unwrap_or(Self::Identifier(Token2::default()))
    }
}

impl<'a> PrattLanguage<'a> for RustExpression {
    type Root = RustAst;
    type TokenKind = RustTokenKind;

    fn parse_atom(state: &mut RustParserState<'a>) -> Option<Self> {
        if state.token_eat_text_sen("(") {
            state.advance();
            let Some(expr) = parse_expr::<Self>(state, 0) else {
                state.record_syntax_error("Error in parsing atom expression");
                return None;
            };

            // it's a tuple now
            if state.token_eat_text_sen(",") {
                let mut values = vec![expr];
                state.advance();

                loop {
                    if state.token_eat_text_sen(")") {
                        break;
                    }

                    let Some(expr2) = parse_expr::<Self>(state, 0) else {
                        state.record_syntax_error(
                            "Failed to parse expr for tuple",
                        );
                        return None;
                    };

                    values.push(expr2);
                    if state.token_eat_text_sen(",") {
                        state.advance();
                    }
                }

                return Some(Self::Tuple(values));
            }

            state.token_expect_text_sen(")");
            return Some(expr);
        }

        if state.token_eat_kinds(&[
            RustTokenKind::String,
            RustTokenKind::Integer,
            RustTokenKind::Float,
        ]) {
            return Some(Self::Literal(state.token_current().unwrap().into()));
        }

        // identifiers and path expressions
        if state.token_eat_kind(RustTokenKind::Identifier) {
            let token2 = Token2::from(state.token_current().unwrap());
            if state.token_peek_at_text(1) == Some("::") {
                state.seek_offset(2);
                let mut path = vec![token2];
                while state.not_at_end() {
                    if state.token_eat_kind(RustTokenKind::Identifier) {
                        path.push(state.token_current().unwrap().into());
                        state.advance();
                    }

                    if !state.token_eat_text_sen("::") {
                        break;
                    }

                    state.advance();
                }

                return Some(Self::Path(path));
            }

            return Some(Self::Identifier(token2));
        }

        state.record_unexpected_token();
        None
    }

    fn parse_prefix(
        state: &mut RustParserState<'a>,
        min_bp: isize,
        op: Token2,
    ) -> Option<Self> {
        Some(Self::UnaryOp {
            op,
            expr: parse_expr::<Self>(state, min_bp)?.into(),
        })
    }

    fn parse_infix(
        state: &mut RustParserState<'a>,
        min_bp: isize,
        op: Token2,
        lhs: Self,
    ) -> Option<Self> {
        Some(Self::BinaryOp {
            op,
            left: lhs.into(),
            right: parse_expr::<Self>(state, min_bp)?.into(),
        })
    }

    // TODO: Later
    fn parse_postfix(
        state: &mut RustParserState<'a>,
        op: Token2,
        lhs: Self,
    ) -> Option<Self> {
        None
    }

    fn prefix_binding(token: &RustToken<'a>) -> Option<PrattAssociativity> {
        match token.text {
            "-" | "!" | "*" | "&" => Some(PrattAssociativity::Right(12)),
            _ => None,
        }
    }

    fn infix_binding(token: &RustToken<'a>) -> Option<PrattAssociativity> {
        match token.text {
            // Multiply
            "*" | "/" | "%" => Some(PrattAssociativity::Left(11)),

            // Add
            "+" | "-" => Some(PrattAssociativity::Left(10)),

            // Bitwise
            "<<" | ">>" => Some(PrattAssociativity::Left(9)),
            "&" => Some(PrattAssociativity::Left(8)),
            "^" => Some(PrattAssociativity::Left(7)),
            "|" => Some(PrattAssociativity::Left(6)),

            // Logical
            "&&" => Some(PrattAssociativity::Left(5)),
            "||" => Some(PrattAssociativity::Left(4)),

            // Comp
            "<" | ">" | "<=" | ">=" => Some(PrattAssociativity::None(3)),

            // Equal
            "==" | "!=" => Some(PrattAssociativity::None(2)),

            // Assignment
            "=" => Some(PrattAssociativity::Right(1)),
            "+=" => Some(PrattAssociativity::Right(1)),
            "-=" => Some(PrattAssociativity::Right(1)),
            "*=" => Some(PrattAssociativity::Right(1)),
            "%=" => Some(PrattAssociativity::Right(1)),
            "/=" => Some(PrattAssociativity::Right(1)),
            "&=" => Some(PrattAssociativity::Right(1)),
            "|=" => Some(PrattAssociativity::Right(1)),
            "^=" => Some(PrattAssociativity::Right(1)),
            "<<=" => Some(PrattAssociativity::Right(1)),
            "=>>" => Some(PrattAssociativity::Right(1)),
            _ => None,
        }
    }

    fn postfix_binding(token: &RustToken<'a>) -> Option<PrattAssociativity> {
        match token.text {
            "." | "(" | "[" | "?" | "{" => Some(PrattAssociativity::Left(14)),
            "as" => Some(PrattAssociativity::Left(13)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RustBlock {
    pub statements: Vec<RustStatement>,
    pub level: usize,
}

#[derive(Debug, Clone)]
pub struct RustMatchArm {
    pub pattern: RustPattern,
    pub guard: Option<RustExpression>,
    pub body: RustExpression,
}

#[derive(Debug, Clone)]
pub struct RustFieldInit {
    pub name: String,
    pub expr: RustExpression,
}
