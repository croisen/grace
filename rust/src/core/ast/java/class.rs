use crate::core::ast::java::annotation::JavaAnnotation;
use crate::core::ast::java::member::JavaMember;
use crate::core::ast::java::type_parameter::JavaTypeParameter;
use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub struct JavaClass {
    pub name: String,
    pub type_parameters: Vec<JavaTypeParameter>,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub members: Vec<JavaMember>,
}
