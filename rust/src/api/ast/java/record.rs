use crate::api::GraceError;
use crate::api::ast::java::annotation::JavaAnnotation;
use crate::api::ast::java::member::JavaMember;
use crate::api::ast::java::parameter::JavaParameter;
use crate::api::ast::java::type_parameter::JavaTypeParameter;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaRecord {
    pub name: String,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub parameters: Option<JavaParameter>,
    pub implements: Vec<String>,
    pub members: Vec<JavaMember>,
}
