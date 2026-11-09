use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub struct JavaParameter {
    pub name: String,
    pub r#type: String,
}
