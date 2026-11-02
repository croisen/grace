use crate::api::ast::java::annotation::JavaAnnotation;
use crate::api::ast::java::parameter::JavaParameter;
use crate::api::ast::java::statement::JavaStatement;
use crate::api::ast::java::type_parameter::JavaTypeParameter;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct JavaMethod {
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
