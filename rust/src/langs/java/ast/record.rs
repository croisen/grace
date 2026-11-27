use crate::core::parser::pratt::Token2;
use crate::langs::java::ast::annotation::JavaAnnotation;
use crate::langs::java::ast::member::JavaMember;
use crate::langs::java::ast::parameter::JavaParameter;

#[derive(Debug, Clone)]
pub struct JavaRecord {
    pub name: Token2,
    pub modifiers: Vec<Token2>,
    pub annotations: Vec<JavaAnnotation>,
    pub parameters: Option<JavaParameter>,
    pub implements: Vec<Token2>,
    pub members: Vec<JavaMember>,
}
