#[derive(Debug, Clone)]
pub struct GenericType {
    pub base_type: Token2,
    pub type_arguments: Vec<Token2>,
}
use crate::core::parser::pratt::Token2;
