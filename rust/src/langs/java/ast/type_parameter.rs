#[derive(Debug, Clone)]
pub struct JavaTypeParameter {
    pub name: Token2,
    pub constraints: Vec<Token2>,
}
use crate::core::parser::pratt::Token2;
