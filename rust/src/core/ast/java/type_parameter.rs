use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub struct JavaTypeParameter {
    pub name: String,
    pub constraints: Vec<String>,
}
