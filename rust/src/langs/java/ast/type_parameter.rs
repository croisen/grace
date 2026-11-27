use crate::core::parser::pratt::Token2;

#[derive(Debug, Clone)]
pub struct JavaTypeParameter {
    pub name: Token2,
    pub constraints: Vec<Token2>,
}
