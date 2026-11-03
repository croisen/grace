use crate::core::ast::java::annotation::JavaAnnotation;
use crate::core::ast::java::parameter::JavaParameter;
use crate::core::ast::java::statement::JavaStatement;
use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub struct JavaConstructor {
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub name: String,
    pub parameters: Vec<JavaParameter>,
    pub body: Vec<JavaStatement>,
}
