use crate::langs::rust::ast::item::alias::RustTypeAlias;
use crate::langs::rust::ast::item::r#const::RustConst;
use crate::langs::rust::ast::item::function::RustFunction;
use crate::langs::rust::ast::r#type::RustType;
use crate::langs::rust::ast::r#type::generics::RustGeneric;

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
