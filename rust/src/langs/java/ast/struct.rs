use crate::core::parser::pratt::Token2;
use crate::langs::java::ast::annotation::JavaAnnotation;
use crate::langs::java::ast::member::JavaMember;
use crate::langs::java::ast::type_parameter::JavaTypeParameter;

#[derive(Debug, Clone)]
pub struct JavaStruct {
    pub name: Token2,
    pub type_parameters: Vec<JavaTypeParameter>,
    pub modifiers: Vec<Token2>,
    pub annotations: Vec<JavaAnnotation>,
    pub implements: Vec<Token2>,
    pub members: Vec<JavaMember>,
}
