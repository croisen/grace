use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub struct GenericType {
    pub base_type: String,
    pub type_arguments: Vec<String>,
}
