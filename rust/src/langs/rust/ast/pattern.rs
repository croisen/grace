use crate::core::parser::pratt::Token2;

#[derive(Debug, Clone)]
pub enum RustPattern {
    Wildcard,
    Identifier(Token2),
    Literal(Token2),
    Tuple(Vec<RustPattern>),
    Struct(Token2, Vec<RustFieldPattern>),
}

#[derive(Debug, Clone)]
pub struct RustFieldPattern {
    pub name: Token2,
    pub pattern: RustPattern,
}
