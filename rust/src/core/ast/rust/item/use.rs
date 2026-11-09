use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub struct RustUse {
    pub main: Vec<String>,
    pub r#final: Vec<RustUse>, // absolute cinema
    pub alias: String,
}
