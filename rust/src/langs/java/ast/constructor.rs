use crate::langs::java::ast::annotation::JavaAnnotation;
use crate::langs::java::ast::parameter::JavaParameter;
use crate::langs::java::ast::statement::JavaStatement;

#[derive(Debug, Clone)]
pub struct JavaConstructor {
    pub modifiers: Vec<Token2>,
    pub annotations: Vec<JavaAnnotation>,
    pub name: Token2,
    pub parameters: Vec<JavaParameter>,
    pub body: Vec<JavaStatement>,
}
use crate::core::parser::pratt::Token2;
