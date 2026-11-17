use crate::core::parser::pratt::{
    PrattAssociativity, PrattLanguage, parse_expr,
};
use crate::langs::rust::ast::pattern::RustPattern;
use crate::langs::rust::ast::statement::RustStatement;
use crate::langs::rust::ast::{RustAst, RustParserState};
use crate::langs::rust::lexer::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub enum RustExpression {
    Identifier(String),
    Literal(String),
    UnaryOp {
        op: String,
        expr: Box<RustExpression>,
    },
    BinaryOp {
        op: String,
        left: Box<RustExpression>,
        right: Box<RustExpression>,
    },
    FunctionCall {
        callee: Box<RustExpression>,
        args: Vec<RustExpression>,
    },
    FieldAccess {
        receiver: Box<RustExpression>,
        field: String,
    },
    Index {
        receiver: Box<RustExpression>,
        index: Box<RustExpression>,
    },
    Parenthesized(Box<RustExpression>),
    Block(Vec<RustBlock>),
    If {
        condition: Box<RustExpression>,
        r#then: RustBlock,
        r#else: Option<RustBlock>,
    },
    While {
        condition: Box<RustExpression>,
        body: RustBlock,
    },
    For {
        var: String,
        iter: Box<RustExpression>,
        body: RustBlock,
    },
    Loop(RustBlock),
    Match(Box<RustExpression>, Vec<RustMatchArm>),
    Tuple(Vec<RustExpression>),
    Array(Vec<RustExpression>),
    Struct(String, Vec<RustFieldInit>),
}

impl RustExpression {
    pub fn new<'a>(state: &mut RustParserState<'a>) -> Self {
        parse_expr::<Self>(state, 0)
            .unwrap_or(Self::Identifier("Placeholder".into()))
    }
}

impl<'a> PrattLanguage<'a> for RustExpression {
    type Root = RustAst;
    type TokenKind = RustTokenKind;

    fn parse_atom(state: &mut RustParserState<'a>) -> Option<Self> {
        None
    }

    fn parse_prefix(
        state: &mut RustParserState<'a>,
        op: String,
        rhs: Self,
    ) -> Option<Self> {
        None
    }

    fn parse_infix(
        state: &mut RustParserState<'a>,
        min_bp: isize,
        op: String,
        lhs: Self,
    ) -> Option<Self> {
        None
    }

    fn parse_postfix(
        state: &mut RustParserState<'a>,
        op: String,
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
            "." | "(" | "[" | "?" => Some(PrattAssociativity::Left(14)),
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
