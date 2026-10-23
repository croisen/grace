use crate::api::GraceError;
use crate::api::ast::java::annotation::JavaAnnotation;
use crate::api::ast::java::expression::JavaExpression;
use crate::api::ast::java::member::JavaMember;
use crate::api::ast::java::type_parameter::JavaTypeParameter;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaEnum {
    pub name: String,
    pub type_parameters: Vec<JavaTypeParameter>,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub implements: Vec<String>,
    pub constants: Vec<JavaEnumConstant>,
    pub members: Vec<JavaMember>,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaEnumConstant {
    pub name: String,
    pub annotations: Vec<JavaAnnotation>,
    pub arguments: Vec<JavaExpression>,
    pub body: Option<Vec<JavaMember>>,
}
