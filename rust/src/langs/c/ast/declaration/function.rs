use crate::langs::c::ast::declaration::spec::CDeclSpec;
use crate::langs::c::ast::declarator::CDeclarator;
use crate::langs::c::ast::statement::CCompoundStatement;

#[derive(Debug, Clone)]
pub struct CFunctionDecl {
    pub declspec: Vec<CDeclSpec>,
    pub decl: CDeclarator,
    pub body: CCompoundStatement,
}
