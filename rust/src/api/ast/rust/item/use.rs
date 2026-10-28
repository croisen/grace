use crate::api::GraceError;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct RustUse {
    pub main: Vec<String>,
    pub r#final: Vec<RustUse>, // absolute cinema
    pub alias: String,
}
