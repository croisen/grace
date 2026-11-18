use crate::langs::c::ast::declaration::spec::CDeclSpec;
use crate::langs::c::ast::declarator::init::CInitDeclarator;

pub mod r#enum;
pub mod function;
pub mod spec;
pub mod r#struct;

#[derive(Debug, Clone)]
pub struct CDecl {
    pub declspec: Vec<CDeclSpec>,
    pub init_decl: Vec<CInitDeclarator>,
}
