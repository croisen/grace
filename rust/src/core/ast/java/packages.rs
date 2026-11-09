use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub struct JavaPackage {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct JavaImport {
    pub path: String,
    pub is_static: bool,
}
