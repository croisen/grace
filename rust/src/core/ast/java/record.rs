use crate::core::ast::java::annotation::JavaAnnotation;
use crate::core::ast::java::member::JavaMember;
use crate::core::ast::java::parameter::JavaParameter;
use crate::core::ast::java::type_parameter::JavaTypeParameter;
use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub struct JavaRecord {
    pub name: String,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub parameters: Option<JavaParameter>,
    pub implements: Vec<String>,
    pub members: Vec<JavaMember>,
}
