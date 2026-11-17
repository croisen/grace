use crate::langs::java::ast::annotation::JavaAnnotation;

#[derive(Debug, Clone)]
pub struct JavaField {
    pub name: String,
    pub r#type: String,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
}
