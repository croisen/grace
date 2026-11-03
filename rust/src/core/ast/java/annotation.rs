use crate::core::ast::java::expression::JavaExpression;
use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub struct JavaAnnotation {
    pub name: String,
    pub arguments: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
pub struct JavaAnnotationType {
    pub name: String,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub elements: Vec<JavaAnnotationElement>,
}

#[derive(Debug, Clone)]
pub struct JavaAnnotationElement {
    pub name: String,
    pub r#type: String,
    pub default_value: Option<JavaExpression>,
}
