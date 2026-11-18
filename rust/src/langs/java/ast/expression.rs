use crate::core::parser::pratt::{
    PrattAssociativity, PrattLanguage, Token2, parse_expr,
};
use crate::langs::java::ast::member::JavaMember;
use crate::langs::java::ast::parameter::JavaParameter;
use crate::langs::java::ast::statement::JavaStatement;
use crate::langs::java::ast::{JavaAst, JavaParserState};
use crate::langs::java::lexer::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub enum JavaExpression {
    Literal(Token2),
    Identifier(Token2),
    MethodCall(JavaMethodCall),
    FieldAccess(JavaFieldAccess),
    ArrayAccess(JavaArrayAccess),
    ArrayCreation(JavaArrayCreation),
    New(JavaNew),
    AnonymousClass(JavaAnonymousClass),
    This,
    Super,
    UnaryOp {
        op: Token2,
        expr: Box<JavaExpression>,
        is_prefix: bool,
    },
    BinaryOp {
        op: Token2,
        left: Box<JavaExpression>,
        right: Box<JavaExpression>,
    },
    Assignment {
        op: Token2,
        left: Box<JavaExpression>,
        right: Box<JavaExpression>,
    },
    Ternary {
        cond: Box<JavaExpression>,
        then: Box<JavaExpression>,
        r#else: Box<JavaExpression>,
    },
    Cast {
        target_type: Token2,
        expression: Box<JavaExpression>,
    },
    Lambda(JavaLambda),
}

impl JavaExpression {
    pub fn new<'a>(state: &mut JavaParserState<'a>) -> Self {
        parse_expr(state, 0).unwrap_or(Self::Identifier(Token2::default()))
    }
}

// impl later
impl<'a> PrattLanguage<'a> for JavaExpression {
    type Root = JavaAst;
    type TokenKind = JavaTokenKind;

    fn parse_atom(state: &mut JavaParserState<'a>) -> Option<Self> {
        None
    }

    fn parse_prefix(
        state: &mut JavaParserState<'a>,
        min_bp: isize,
        op: Token2,
    ) -> Option<Self> {
        None
    }

    fn parse_infix(
        state: &mut JavaParserState<'a>,
        min_bp: isize,
        op: Token2,
        lhs: Self,
    ) -> Option<Self> {
        None
    }

    fn parse_postfix(
        state: &mut JavaParserState<'a>,
        op: Token2,
        lhs: Self,
    ) -> Option<Self> {
        None
    }

    fn prefix_binding(token: &JavaToken<'a>) -> Option<PrattAssociativity> {
        None
    }

    fn infix_binding(token: &JavaToken<'a>) -> Option<PrattAssociativity> {
        None
    }

    fn postfix_binding(token: &JavaToken<'a>) -> Option<PrattAssociativity> {
        None
    }
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
