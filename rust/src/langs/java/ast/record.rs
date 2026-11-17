use crate::langs::java::ast::annotation::JavaAnnotation;
use crate::langs::java::ast::member::JavaMember;
use crate::langs::java::ast::parameter::JavaParameter;

#[derive(Debug, Clone)]
pub struct JavaRecord {
    pub name: String,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
    pub parameters: Option<JavaParameter>,
    pub implements: Vec<String>,
    pub members: Vec<JavaMember>,
}
