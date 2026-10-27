use crate::api::GraceError;
use crate::api::ast::java::annotation::JavaAnnotation;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct JavaField {
    pub name: String,
    pub r#type: String,
    pub modifiers: Vec<String>,
    pub annotations: Vec<JavaAnnotation>,
}
