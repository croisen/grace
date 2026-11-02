use crate::api::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub(crate) struct RustLifetime(String);
