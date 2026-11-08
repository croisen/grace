use crate::core::ast::rust::pattern::RustPattern;
use crate::core::ast::rust::statement::RustStatement;
use crate::core::ast::rust::{RustAst, RustParserState};
use crate::core::lexers::rust::{RustToken, RustTokenKind};
use crate::core::parser::pratt::{PrattLanguage, parse_expr};

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
        parse_expr(state, 0).unwrap_or(Self::Identifier("Placeholder".into()))
    }
}

// impl later
impl<'a> PrattLanguage<'a> for RustExpression {
    type Root = RustAst;
    type TokenKind = RustTokenKind;
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
