use crate::api::GraceError;
use crate::api::ast::rust::item::RustItem;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

pub(crate) struct RustModule {
    pub name: String,
    pub items: Vec<RustItem>,
}
