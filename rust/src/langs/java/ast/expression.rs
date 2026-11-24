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
        if state.token_eat_text_sen("(") {
            state.advance();
            let expr = parse_expr::<Self>(state, 0)?;
            state.token_expect_take_text_sen(")");
            return Some(expr);
        }
        if state.token_text() == Some("this") {
            return Some(Self::This);
        }
        if state.token_text() == Some("super") {
            return Some(Self::Super);
        }
        if state.token_text() == Some("new") {
            state.advance();
            let r#type: Token2 = state.token_current()?.into();
            state.advance();
            state.token_expect_text_sen("(");
            state.advance();
            let mut arguments = Vec::new();
            while state.not_at_end() && !state.token_eat_text_sen(")") {
                arguments.push(parse_expr::<Self>(state, 0)?);
                if !state.token_eat_text_sen(",") {
                    break;
                }
                state.advance();
            }
            state.token_expect_take_text_sen(")");
            return Some(Self::New(JavaNew { r#type, arguments }));
        }
        let token = state.token_current()?;
        if matches!(
            &token.kind,
            JavaTokenKind::Identifier
                | JavaTokenKind::Integer
                | JavaTokenKind::Float
                | JavaTokenKind::String
        ) {
            let token = token.into();
            return Some(
                if matches!(state.token_kind(), Some(kind) if *kind == JavaTokenKind::Identifier)
                {
                    Self::Identifier(token)
                } else {
                    Self::Literal(token)
                },
            );
        }
        state.record_unexpected_token();
        None
    }

    fn parse_prefix(
        state: &mut JavaParserState<'a>,
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
        state: &mut JavaParserState<'a>,
        min_bp: isize,
        op: Token2,
        lhs: Self,
    ) -> Option<Self> {
        let right = parse_expr::<Self>(state, min_bp)?;
        if matches!(op.text.as_str(), "=" | "+=" | "-=" | "*=" | "/=" | "%=") {
            Some(Self::Assignment { op, left: lhs.into(), right: right.into() })
        } else {
            Some(Self::BinaryOp { op, left: lhs.into(), right: right.into() })
        }
    }

    fn parse_postfix(
        state: &mut JavaParserState<'a>,
        op: Token2,
        lhs: Self,
    ) -> Option<Self> {
        if op.text == "(" {
            let mut arguments = Vec::new();
            while state.not_at_end() && !state.token_eat_text_sen(")") {
                arguments.push(parse_expr::<Self>(state, 0)?);
                if !state.token_eat_text_sen(",") {
                    break;
                }
                state.advance();
            }
            state.token_expect_take_text_sen(")");
            let (target, name) = match lhs {
                Self::Identifier(name) => (None, name),
                Self::FieldAccess(access) => {
                    (Some(access.target), access.field)
                }
                _ => return None,
            };
            Some(Self::MethodCall(JavaMethodCall { target, name, arguments }))
        } else if op.text == "[" {
            let index = parse_expr::<Self>(state, 0)?;
            state.token_expect_take_text_sen("]");
            Some(Self::ArrayAccess(JavaArrayAccess {
                target: lhs.into(),
                index: index.into(),
            }))
        } else if op.text == "." {
            let field: Token2 = state.token_current()?.into();
            state.advance();
            Some(Self::FieldAccess(JavaFieldAccess {
                target: lhs.into(),
                field,
            }))
        } else if op.text == "?" {
            let then = parse_expr::<Self>(state, 0)?;
            state.token_expect_text_sen(":");
            state.advance();
            let r#else = parse_expr::<Self>(state, 0)?;
            Some(Self::Ternary {
                cond: lhs.into(),
                then: then.into(),
                r#else: r#else.into(),
            })
        } else if op.text == "++" || op.text == "--" {
            Some(Self::UnaryOp { op, expr: lhs.into(), is_prefix: false })
        } else {
            None
        }
    }

    fn prefix_binding(token: &JavaToken<'a>) -> Option<PrattAssociativity> {
        match token.text {
            "!" | "~" | "+" | "-" | "++" | "--" => {
                Some(PrattAssociativity::Right(9))
            }
            _ => None,
        }
    }

    fn infix_binding(token: &JavaToken<'a>) -> Option<PrattAssociativity> {
        match token.text {
            "*" | "/" | "%" => Some(PrattAssociativity::Left(8)),
            "+" | "-" => Some(PrattAssociativity::Left(7)),
            "<" | ">" | "<=" | ">=" | "instanceof" => {
                Some(PrattAssociativity::None(6))
            }
            "==" | "!=" => Some(PrattAssociativity::None(5)),
            "&" => Some(PrattAssociativity::Left(4)),
            "^" => Some(PrattAssociativity::Left(3)),
            "|" => Some(PrattAssociativity::Left(2)),
            "&&" => Some(PrattAssociativity::Left(1)),
            "||" => Some(PrattAssociativity::Left(0)),
            "=" | "+=" | "-=" | "*=" | "/=" | "%=" => {
                Some(PrattAssociativity::Right(-1))
            }
            _ => None,
        }
    }

    fn postfix_binding(token: &JavaToken<'a>) -> Option<PrattAssociativity> {
        match token.text {
            "(" | "[" | "." | "++" | "--" => Some(PrattAssociativity::Left(10)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JavaMethodCall {
    pub target: Option<Box<JavaExpression>>,
    pub name: Token2,
    pub arguments: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
pub struct JavaFieldAccess {
    pub target: Box<JavaExpression>,
    pub field: Token2,
}

#[derive(Debug, Clone)]
pub struct JavaArrayAccess {
    pub target: Box<JavaExpression>,
    pub index: Box<JavaExpression>,
}

#[derive(Debug, Clone)]
pub struct JavaArrayCreation {
    pub r#type: Token2,
    pub dim: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
pub struct JavaNew {
    pub r#type: Token2,
    pub arguments: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
pub struct JavaAnonymousClass {
    pub super_type: Token2,
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
