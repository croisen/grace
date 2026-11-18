use crate::core::parser::pratt::Token2;
use crate::langs::c::ast::declaration::CDecl;
use crate::langs::c::ast::expression::CExpression;

#[derive(Debug, Clone)]
pub enum CStatement {
    Labeled(CLabeledStatement),
    Compound(CCompoundStatement),
    If {
        cond: Box<CExpression>,
        r#then: Box<CStatement>,
        r#else: Option<Box<CStatement>>,
    },
    Switch {
        expr: Box<CExpression>,
        statement: Box<CStatement>,
    },
    While {
        cond: Box<CExpression>,
        statement: Box<CStatement>,
    },
    DoWhile {
        cond: Box<CExpression>,
        statement: Box<CStatement>,
    },
    For {
        init: Option<Box<CExpression>>,
        cond: Option<Box<CExpression>>,
        update: Option<Box<CExpression>>,
        statement: Box<CStatement>,
    },
    Goto(Token2),
    Continue,
    Break,
    Return(Option<CExpression>),
}

#[derive(Debug, Clone)]
pub enum CLabeledStatement {
    Label { name: Token2, statement: Box<CStatement> },
    Case { constexpr: CExpression, statement: Box<CStatement> },
    Default { statement: Box<CStatement> },
}

#[derive(Debug, Clone)]
pub struct CCompoundStatement(Vec<CBlockItem>);

#[derive(Debug, Clone)]
pub enum CBlockItem {
    Declaration(CDecl),
    Statement(CStatement),
}
