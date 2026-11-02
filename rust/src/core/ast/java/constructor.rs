use crate::api::ast::java::annotation::JavaAnnotation;
use crate::api::ast::java::parameter::JavaParameter;
use crate::api::ast::java::statement::JavaStatement;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct JavaConstructor {
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub name: String,
    pub parameters: Vec<JavaParameter>,
    pub body: Vec<JavaStatement>,
}
