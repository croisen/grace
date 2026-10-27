use crate::api::GraceError;
use crate::api::ast::rust::item::alias::RustTypeAlias;
use crate::api::ast::rust::item::function::RustFunction;
use crate::api::ast::rust::r#type::generics::RustGeneric;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

pub(crate) struct RustTrait {
    pub name: String,
    pub generics: Option<RustGeneric>,
    pub items: Vec<RustTraitItem>,
}

pub(crate) enum RustTraitItem {
    Method(RustFunction),
    Type(RustTypeAlias),
}
