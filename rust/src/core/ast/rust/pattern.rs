use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub enum RustPattern {
    Wildcard,
    Identifier(String),
    Literal(String),
    Tuple(Vec<RustPattern>),
    Struct(String, Vec<RustFieldPattern>),
}

#[derive(Debug, Clone)]
pub struct RustFieldPattern {
    pub name: String,
    pub pattern: RustPattern,
}
