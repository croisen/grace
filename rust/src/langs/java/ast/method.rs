use crate::langs::java::ast::annotation::JavaAnnotation;
use crate::langs::java::ast::parameter::JavaParameter;
use crate::langs::java::ast::statement::JavaStatement;
use crate::langs::java::ast::type_parameter::JavaTypeParameter;

#[derive(Debug, Clone)]
pub struct JavaMethod {
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
