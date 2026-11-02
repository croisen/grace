use crate::api::ast::java::expression::JavaExpression;
use crate::api::ast::java::parameter::JavaParameter;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) enum JavaStatement {
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
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct JavaTryStatement {
    pub block: Vec<JavaStatement>,
    pub catches: Vec<JavaCatchClause>,
    pub finally: Option<Vec<JavaStatement>>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct JavaCatchClause {
    pub parameter: JavaParameter,
    pub block: Vec<JavaStatement>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct JavaSwitchCase {
    pub label: String,
    pub body: Vec<JavaStatement>,
}
