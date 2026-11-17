use crate::langs::java::ast::annotation::JavaAnnotation;
use crate::langs::java::ast::expression::JavaExpression;
use crate::langs::java::ast::member::JavaMember;
use crate::langs::java::ast::type_parameter::JavaTypeParameter;

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
