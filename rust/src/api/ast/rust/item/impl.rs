use crate::api::GraceError;
use crate::api::ast::rust::item::alias::RustTypeAlias;
use crate::api::ast::rust::item::r#const::RustConst;
use crate::api::ast::rust::item::function::RustFunction;
use crate::api::ast::rust::r#type::RustType;
use crate::api::ast::rust::r#type::generics::RustGeneric;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct RustImpl {
    pub generics: Option<RustGeneric>,
    pub r#trait: Option<RustType>,
    pub r#type: RustType,
    pub items: Vec<RustImplItem>,
}

#[derive(Debug, Clone)]
pub(crate) enum RustImplItem {
    Method(RustFunction),
    Type(RustTypeAlias),
    Const(RustConst),
}
