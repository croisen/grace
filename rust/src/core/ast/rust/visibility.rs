use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub enum RustVisibility {
    Pub,
    PubCrate,
    PubSelf,
    PubSuper,
    PubIn(Vec<String>),
}
