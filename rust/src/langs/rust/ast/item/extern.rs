use crate::langs::rust::ast::item::RustItem;

#[derive(Debug, Clone)]
pub struct RustExtern {
    pub abi: Option<String>,
    pub items: Vec<RustItem>,
}
