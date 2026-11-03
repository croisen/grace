use crate::core::ast::java::annotation::JavaAnnotation;
use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub struct JavaField {
    pub name: String,
    pub r#type: String,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
}
