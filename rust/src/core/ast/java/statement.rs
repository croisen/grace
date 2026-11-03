use crate::core::ast::java::expression::JavaExpression;
use crate::core::ast::java::parameter::JavaParameter;
use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub enum JavaStatement {
    Expression(JavaExpression),
    Return(Option<JavaExpression>),
    Block(Vec<JavaExpression>),
    Try(JavaTryStatement),
    Throw(JavaExpression),
    If {
        condition: JavaExpression,
        then: Box<JavaStatement>,
        r#else: Option<Box<JavaStatement>>,
    },
    While {
        condition: JavaExpression,
        body: Box<JavaStatement>,
    },
    DoWhile {
        condition: JavaExpression,
        body: Box<JavaStatement>,
    },
    For {
        init: Option<Box<JavaStatement>>,
        condition: Option<JavaExpression>,
        update: Option<JavaExpression>,
        body: Box<JavaStatement>,
    },
    ForEach {
        r#type: String,
        name: String,
        iter: JavaExpression,
        body: Box<JavaStatement>,
    },
    Switch {
        selector: JavaExpression,
        cases: Vec<JavaSwitchCase>,
        default: Vec<JavaStatement>,
    },
    Break,
    Continue,
    LocalVariable {
        name: String,
        r#type: String,
        init: Option<JavaExpression>,
    },
}

#[derive(Debug, Clone)]
pub struct JavaTryStatement {
    pub block: Vec<JavaStatement>,
    pub catches: Vec<JavaCatchClause>,
    pub finally: Option<Vec<JavaStatement>>,
}

#[derive(Debug, Clone)]
pub struct JavaCatchClause {
    pub parameter: JavaParameter,
    pub block: Vec<JavaStatement>,
}

#[derive(Debug, Clone)]
pub struct JavaSwitchCase {
    pub label: String,
    pub body: Vec<JavaStatement>,
}
