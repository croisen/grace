use crate::langs::java::ast::annotation::JavaAnnotation;
use crate::langs::java::ast::parameter::JavaParameter;
use crate::langs::java::ast::statement::JavaStatement;
use crate::langs::java::ast::type_parameter::JavaTypeParameter;

#[derive(Debug, Clone)]
pub struct JavaMethod {
    pub name: Token2,
    pub type_parameters: Vec<JavaTypeParameter>,
    pub modifiers: Vec<Token2>,
    pub annotations: Vec<JavaAnnotation>,
    pub return_type: Token2,
    pub parameters: Vec<JavaParameter>,
    pub body: Vec<JavaStatement>,
    pub throws: Vec<Token2>,
    pub is_static: bool,
}
use crate::core::parser::pratt::Token2;
