use crate::langs::java::ast::expression::JavaExpression;
use crate::langs::java::ast::parameter::JavaParameter;

#[derive(Debug, Clone)]
pub enum JavaStatement {
    Expression(JavaExpression),
    Return(Option<JavaExpression>),
    Block(Vec<JavaStatement>),
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
        r#type: Token2,
        name: Token2,
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
        name: Token2,
        r#type: Token2,
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
    pub label: Token2,
    pub body: Vec<JavaStatement>,
}
use crate::core::parser::pratt::Token2;
