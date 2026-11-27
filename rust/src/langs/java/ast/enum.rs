use crate::core::parser::pratt::Token2;
use crate::langs::java::ast::annotation::JavaAnnotation;
use crate::langs::java::ast::expression::JavaExpression;
use crate::langs::java::ast::member::JavaMember;
use crate::langs::java::ast::type_parameter::JavaTypeParameter;

#[derive(Debug, Clone)]
pub struct JavaEnum {
    pub name: Token2,
    pub type_parameters: Vec<JavaTypeParameter>,
    pub modifiers: Vec<Token2>,
    pub annotations: Vec<JavaAnnotation>,
    pub implements: Vec<Token2>,
    pub constants: Vec<JavaEnumConstant>,
    pub members: Vec<JavaMember>,
}

#[derive(Debug, Clone)]
pub struct JavaEnumConstant {
    pub name: Token2,
    pub annotations: Vec<JavaAnnotation>,
    pub arguments: Vec<JavaExpression>,
    pub body: Option<Vec<JavaMember>>,
}
