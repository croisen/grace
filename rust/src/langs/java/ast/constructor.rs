use crate::langs::java::ast::annotation::JavaAnnotation;
use crate::langs::java::ast::parameter::JavaParameter;
use crate::langs::java::ast::statement::JavaStatement;

#[derive(Debug, Clone)]
pub struct JavaConstructor {
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub name: String,
    pub parameters: Vec<JavaParameter>,
    pub body: Vec<JavaStatement>,
}
