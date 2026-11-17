use crate::langs::java::ast::annotation::JavaAnnotation;
use crate::langs::java::ast::member::JavaMember;
use crate::langs::java::ast::type_parameter::JavaTypeParameter;

#[derive(Debug, Clone)]
pub struct JavaInterface {
    pub name: String,
    pub type_parameters: Vec<JavaTypeParameter>,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub extends: Vec<String>,
    pub members: Vec<JavaMember>,
}
