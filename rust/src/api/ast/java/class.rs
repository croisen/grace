use crate::api::GraceError;
use crate::api::ast::java::annotation::JavaAnnotation;
use crate::api::ast::java::member::JavaMember;
use crate::api::ast::java::type_parameter::JavaTypeParameter;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaClass {
    pub name: String,
    pub type_parameters: Vec<JavaTypeParameter>,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub members: Vec<JavaMember>,
}
