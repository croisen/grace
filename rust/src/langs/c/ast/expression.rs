use crate::core::parser::pratt::Token2;

#[derive(Debug, Clone)]
pub enum CExpression {
    Identifier(Token2),
    Literal(Token2), // raw integers that can be used for enums
    Literal2(Token2),
    Index {
        receiver: Box<CExpression>,
        index: Box<CExpression>,
    },
    FunctionCall {
        callee: Box<CExpression>,
        args: Vec<CExpression>,
    },
    FieldAccess {
        receiver: Box<CExpression>,
        member: Token2,
        is_pointer: bool,
    },
    UnaryOp {
        op: Token2,
        expr: Box<CExpression>,
        is_prefix: bool,
    },
    Cast {
        r#type: (),
        expr: Box<CExpression>,
    },
    BinaryOp {
        op: Token2,
        left: Box<CExpression>,
        right: Box<CExpression>,
    },
    Ternary {
        cond: Box<CExpression>,
        r#then: Box<CExpression>,
        r#else: Box<CExpression>,
    },
    Comma(Vec<CExpression>),
}
