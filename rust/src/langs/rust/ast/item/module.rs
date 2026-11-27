use crate::core::parser::pratt::Token2;
use crate::langs::rust::ast::item::RustItem;

#[derive(Debug, Clone)]
pub struct RustModule {
    pub name: Token2,
    pub items: Vec<RustItem>,
}
