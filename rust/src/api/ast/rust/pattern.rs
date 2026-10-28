use crate::api::GraceError;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) enum RustPattern {
    Wildcard,
    Identifier(String),
    Literal(String),
    Tuple(Vec<RustPattern>),
    Struct(String, Vec<RustFieldPattern>),
}

#[derive(Debug, Clone)]
pub(crate) struct RustFieldPattern {
    pub name: String,
    pub pattern: RustPattern,
}
