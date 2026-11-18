use crate::core::parser::pratt::Token2;
use crate::langs::c::ast::expression::CExpression;

#[derive(Debug, Clone)]
pub struct CEnumDecl {
    pub name: Option<Token2>,
    pub variants: Vec<CEnumVariant>,
}

#[derive(Debug, Clone)]
pub struct CEnumVariant {
    pub identifier: Token2,
    pub constant: Option<CExpression>,
}
