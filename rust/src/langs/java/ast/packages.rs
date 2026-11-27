use crate::core::parser::pratt::Token2;

#[derive(Debug, Clone)]
pub struct JavaPackage {
    pub name: Vec<Token2>,
}

#[derive(Debug, Clone)]
pub struct JavaImport {
    pub path: Vec<Token2>,
    pub is_static: bool,
}
