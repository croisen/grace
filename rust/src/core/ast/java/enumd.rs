use crate::core::ast::java::annotation::JavaAnnotation;
use crate::core::ast::java::expression::JavaExpression;
use crate::core::ast::java::member::JavaMember;
use crate::core::ast::java::type_parameter::JavaTypeParameter;
use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
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
pub struct JavaEnumConstant {
    pub name: String,
    pub annotations: Vec<JavaAnnotation>,
    pub arguments: Vec<JavaExpression>,
    pub body: Option<Vec<JavaMember>>,
}
