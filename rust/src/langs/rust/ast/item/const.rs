use crate::langs::rust::ast::expression::RustExpression;
use crate::langs::rust::ast::r#type::RustType;

#[derive(Debug, Clone)]
pub struct RustConst {
    pub name: String,
    pub r#type: RustType,
    pub expr: RustExpression,
}
