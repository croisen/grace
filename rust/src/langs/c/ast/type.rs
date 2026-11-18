use crate::core::parser::pratt::Token2;
use crate::langs::c::ast::expression::CExpression;

#[derive(Debug, Clone)]
pub struct CTypeName {}

#[derive(Debug, Clone)]
pub enum CType {
    BaseType(Token2),
    Pointer { r#const: bool, r#type: Box<CType> },
    Array { element: Box<CType>, size: Option<CExpression> },
    Function { r#return: Box<CType>, params: Vec<CType>, variadic: bool },
}
