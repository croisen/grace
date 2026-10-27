use crate::api::GraceError;
use crate::api::ast::rust::pattern::RustPattern;
use crate::api::ast::rust::statement::RustStatement;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

pub(crate) enum RustExpression {
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

pub(crate) struct RustBlock {
    pub statements: Vec<RustStatement>,
    pub level: usize,
}

pub(crate) struct RustMatchArm {
    pub pattern: RustPattern,
    pub guard: Option<RustExpression>,
    pub body: RustExpression,
}

pub(crate) struct RustFieldInit {
    pub name: String,
    pub expr: RustExpression,
}
