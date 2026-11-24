#[derive(Debug, Clone)]
pub struct JavaParameter {
    pub name: Token2,
    pub r#type: Token2,
    pub is_varargs: bool,
}
use crate::core::parser::pratt::Token2;
