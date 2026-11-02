use crate::api::ast::java::annotation::JavaAnnotation;
use crate::api::ast::java::member::JavaMember;
use crate::api::ast::java::type_parameter::JavaTypeParameter;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct JavaInterface {
    pub name: String,
    pub type_parameters: Vec<JavaTypeParameter>,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub extends: Vec<String>,
    pub members: Vec<JavaMember>,
}
