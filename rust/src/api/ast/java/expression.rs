use crate::api::GraceError;
use crate::api::ast::java::member::JavaMember;
use crate::api::ast::java::parameter::JavaParameter;
use crate::api::ast::java::statement::JavaStatement;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
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

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaMethodCall {
    pub target: Option<Box<JavaExpression>>,
    pub name: String,
    pub arguments: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaFieldAccess {
    pub target: Box<JavaExpression>,
    pub field: String,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaArrayAccess {
    pub target: Box<JavaExpression>,
    pub index: Box<JavaExpression>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaArrayCreation {
    pub r#type: String,
    pub dim: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaNew {
    pub r#type: String,
    pub arguments: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaAnonymousClass {
    pub super_type: String,
    pub arguments: Vec<JavaExpression>,
    pub members: Vec<JavaMember>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaLambda {
    pub parameters: Vec<JavaParameter>,
    pub body: JavaLambdaBody,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub enum JavaLambdaBody {
    Expression(Box<JavaExpression>),
    Block(Vec<JavaStatement>),
}
