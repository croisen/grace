use crate::langs::rust::ast::item::RustItem;

#[derive(Debug, Clone)]
pub struct RustModule {
    pub name: String,
    pub items: Vec<RustItem>,
}
