use crate::api::ast::java::expression::JavaExpression;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub(crate) struct JavaAnnotation {
    pub name: String,
    pub arguments: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
pub(crate) struct JavaAnnotationType {
    pub name: String,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub elements: Vec<JavaAnnotationElement>,
}

#[derive(Debug, Clone)]
pub(crate) struct JavaAnnotationElement {
    pub name: String,
    pub r#type: String,
    pub default_value: Option<JavaExpression>,
}
