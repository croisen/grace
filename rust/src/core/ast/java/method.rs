use crate::core::ast::java::annotation::JavaAnnotation;
use crate::core::ast::java::parameter::JavaParameter;
use crate::core::ast::java::statement::JavaStatement;
use crate::core::ast::java::type_parameter::JavaTypeParameter;
use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub struct JavaMethod {
    pub name: String,
    pub type_parameters: Vec<JavaTypeParameter>,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub return_type: String,
    pub parameters: Vec<JavaParameter>,
    pub body: Vec<JavaStatement>,
    pub throws: Vec<String>,
    pub is_static: bool,
}
