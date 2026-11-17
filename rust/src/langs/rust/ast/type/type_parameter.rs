use crate::langs::rust::ast::expression::RustExpression;
use crate::langs::rust::ast::r#type::RustType;
use crate::langs::rust::ast::r#type::lifetime::RustLifetime;

#[derive(Debug, Clone)]
pub struct RustTypeParameter {
    pub name: String,
    pub r#type: RustType,
    pub r#mut: bool,
}
