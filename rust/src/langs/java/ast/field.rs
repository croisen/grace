use crate::langs::java::ast::annotation::JavaAnnotation;

#[derive(Debug, Clone)]
pub struct JavaField {
    pub name: Token2,
    pub r#type: Token2,
    pub modifiers: Vec<Token2>,
    pub annotations: Vec<JavaAnnotation>,
}
use crate::core::parser::pratt::Token2;
