use crate::core::parser::pratt::Token2;
use crate::langs::rust::ast::item::alias::RustTypeAlias;
use crate::langs::rust::ast::item::function::RustFunction;
use crate::langs::rust::ast::r#type::generics::RustGeneric;

#[derive(Debug, Clone)]
pub struct RustTrait {
    pub name: Token2,
    pub generics: Option<RustGeneric>,
    pub items: Vec<RustTraitItem>,
}

#[derive(Debug, Clone)]
pub enum RustTraitItem {
    Method(RustFunction),
    Type(RustTypeAlias),
}
