use crate::core::parser::pratt::Token2;
use crate::langs::c::ast::declaration::spec::CTypeQualifier;
use crate::langs::c::ast::declarator::parameter::CParameterList;
use crate::langs::c::ast::expression::CExpression;

pub mod init;
pub mod parameter;

#[derive(Debug, Clone)]
pub struct CDeclarator {
    pub ptr: Option<CPointer>,
    pub direct: CDirectDeclarator,
}

#[derive(Debug, Clone)]
pub struct CPointer {
    pub type_qualifiers: Vec<CTypeQualifier>,
    pub ptr: Option<Box<CPointer>>,
}

#[derive(Debug, Clone)]
pub enum CDirectDeclarator {
    Identifier(Token2),
    Declarator(Box<CDeclarator>),
    Array {
        direct: Box<CDirectDeclarator>,
        type_qualifiers: Vec<CTypeQualifier>,
        assignment_expr: Option<Box<CExpression>>,
    },
    Function {
        direct: Box<CDirectDeclarator>,
        params: CParameterList,
    },
}

#[derive(Debug, Clone)]
pub struct CAbstractDeclarator {
    pub ptr: Option<CPointer>,
    pub direct: Option<Box<CDirectAbstractDeclarator>>,
}

#[derive(Debug, Clone)]
pub enum CDirectAbstractDeclarator {
    AbstractDeclarator(Box<CAbstractDeclarator>),
    Array {
        declarator: Option<Box<CDirectAbstractDeclarator>>,
        params: Option<CParameterList>,
    },
}
