use crate::api::GraceError;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

pub(crate) enum RustVisibility {
    Pub,
    PubCrate,
    PubSelf,
    PubSuper,
    PubIn(Vec<String>),
}
