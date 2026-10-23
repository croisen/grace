use crate::api::GraceError;
use crate::api::ast::java::expression::JavaExpression;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaAnnotation {
    pub name: String,
    pub arguments: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaAnnotationType {
    pub name: String,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub elements: Vec<JavaAnnotationElement>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaAnnotationElement {
    pub name: String,
    pub r#type: String,
    pub default_value: Option<JavaExpression>,
}
