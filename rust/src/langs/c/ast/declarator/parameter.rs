use crate::langs::c::ast::declaration::spec::CDeclSpec;
use crate::langs::c::ast::declarator::CDeclarator;
use crate::langs::c::ast::expression::CExpression;

#[derive(Debug, Clone)]
pub struct CParameterList {
    pub params: Vec<CParameter>,
    pub variadic: bool,
}

#[derive(Debug, Clone)]
pub struct CParameter {
    pub declspec: Vec<CDeclSpec>,
    pub declarator: Option<CDeclarator>,
}

#[derive(Debug, Clone)]
pub struct CStructDeclarator {
    pub decl: Option<CDeclarator>,
    pub bits: Option<CExpression>,
}
