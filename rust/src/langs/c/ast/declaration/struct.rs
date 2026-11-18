use crate::langs::c::ast::declaration::spec::CSpecQualifier;
use crate::langs::c::ast::declarator::parameter::CStructDeclarator;

#[derive(Debug, Clone)]
pub struct CStructDecl {
    pub spec_qualifiers: Vec<CSpecQualifier>,
    pub decls: Vec<CStructDeclarator>,
}
