use crate::core::parser::pratt::Token2;

#[derive(Debug, Clone)]
pub struct RustLifetime(pub Token2);

impl RustLifetime {
    pub fn new(token: Token2) -> Self {
        Self(token)
    }
}
