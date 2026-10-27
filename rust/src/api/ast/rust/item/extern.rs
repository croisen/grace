use crate::api::GraceError;
use crate::api::ast::rust::item::RustItem;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

pub(crate) struct RustExtern {
    pub abi: Option<String>,
    pub items: Vec<RustItem>,
}
