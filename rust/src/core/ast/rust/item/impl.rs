use crate::core::ast::rust::item::alias::RustTypeAlias;
use crate::core::ast::rust::item::r#const::RustConst;
use crate::core::ast::rust::item::function::RustFunction;
use crate::core::ast::rust::r#type::RustType;
use crate::core::ast::rust::r#type::generics::RustGeneric;
use crate::core::lexers::rust::{RustToken, RustTokenKind};

#[derive(Debug, Clone)]
pub struct RustImpl {
    pub generics: Option<RustGeneric>,
    pub r#trait: Option<RustType>,
    pub r#type: RustType,
    pub items: Vec<RustImplItem>,
}

#[derive(Debug, Clone)]
pub enum RustImplItem {
    Method(RustFunction),
    Type(RustTypeAlias),
    Const(RustConst),
}
