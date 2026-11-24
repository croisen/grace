use crate::core::parser::pratt::Token2;

#[derive(Debug, Clone)]
pub struct RustUse {
    pub main: Vec<Token2>,
    pub r#final: Vec<RustUse>, // absolute cinema
    pub alias: Token2,
}
