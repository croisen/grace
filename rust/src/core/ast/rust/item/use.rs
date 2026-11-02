use crate::api::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub(crate) struct RustUse {
    pub main: Vec<String>,
    pub r#final: Vec<RustUse>, // absolute cinema
    pub alias: String,
}
