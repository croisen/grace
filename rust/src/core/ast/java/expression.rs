use crate::core::ast::java::member::JavaMember;
use crate::core::ast::java::parameter::JavaParameter;
use crate::core::ast::java::statement::JavaStatement;
use crate::core::ast::java::{JavaAst, JavaParserState};
use crate::core::lexers::java::{JavaToken, JavaTokenKind};
use crate::core::parser::pratt::{PrattLanguage, parse_expr};

#[derive(Debug, Clone)]
pub enum JavaExpression {
    Literal(String),
    Identifier(String),
    MethodCall(JavaMethodCall),
    FieldAccess(JavaFieldAccess),
    ArrayAccess(JavaArrayAccess),
    ArrayCreation(JavaArrayCreation),
    New(JavaNew),
    AnonymousClass(JavaAnonymousClass),
    This,
    Super,
    UnaryOp {
        op: String,
        expr: Box<JavaExpression>,
        is_prefix: bool,
    },
    BinaryOp {
        op: String,
        left: Box<JavaExpression>,
        right: Box<JavaExpression>,
    },
    Assignment {
        op: String,
        left: Box<JavaExpression>,
        right: Box<JavaExpression>,
    },
    Ternary {
        cond: Box<JavaExpression>,
        then: Box<JavaExpression>,
        r#else: Box<JavaExpression>,
    },
    Cast {
        target_type: String,
        expression: Box<JavaExpression>,
    },
    Lambda(JavaLambda),
}

impl JavaExpression {
    pub fn new<'a>(state: &mut JavaParserState<'a>) -> Self {
        parse_expr(state, 0).unwrap_or(Self::Identifier("Placeholder".into()))
    }
}

// impl later
impl<'a> PrattLanguage<'a> for JavaExpression {
    type Root = JavaAst;
    type TokenKind = JavaTokenKind;
}

#[derive(Debug, Clone)]
pub struct JavaMethodCall {
    pub target: Option<Box<JavaExpression>>,
    pub name: String,
    pub arguments: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
pub struct JavaFieldAccess {
    pub target: Box<JavaExpression>,
    pub field: String,
}

#[derive(Debug, Clone)]
pub struct JavaArrayAccess {
    pub target: Box<JavaExpression>,
    pub index: Box<JavaExpression>,
}

#[derive(Debug, Clone)]
pub struct JavaArrayCreation {
    pub r#type: String,
    pub dim: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
pub struct JavaNew {
    pub r#type: String,
    pub arguments: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
pub struct JavaAnonymousClass {
    pub super_type: String,
    pub arguments: Vec<JavaExpression>,
    pub members: Vec<JavaMember>,
}

#[derive(Debug, Clone)]
pub struct JavaLambda {
    pub parameters: Vec<JavaParameter>,
    pub body: JavaLambdaBody,
}

#[derive(Debug, Clone)]
pub enum JavaLambdaBody {
    Expression(Box<JavaExpression>),
    Block(Vec<JavaStatement>),
}
