use crate::langs::c::ast::declaration::CDecl;
use crate::langs::c::ast::expression::CExpression;

#[derive(Debug, Clone)]
pub struct CInitDeclarator {
    pub decl: CDecl,
    pub init: CInitializer,
}

#[derive(Debug, Clone)]
pub enum CInitializer {
    Assignment(CExpression),
    List(Vec<CInitializer>),
}
