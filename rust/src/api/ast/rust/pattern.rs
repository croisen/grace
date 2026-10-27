use crate::api::GraceError;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

pub(crate) enum RustPattern {
    Wildcard,
    Identifier(String),
    Literal(String),
    Tuple(Vec<RustPattern>),
    Struct(String, Vec<RustFieldPattern>),
}

pub(crate) struct RustFieldPattern {
    pub name: String,
    pub pattern: RustPattern,
}
