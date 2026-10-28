use crate::api::GraceError;
use crate::api::ast::rust::expression::RustExpression;
use crate::api::ast::rust::r#type::RustType;
use crate::api::ast::rust::r#type::lifetime::RustLifetime;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
pub(crate) struct RustGeneric {
    pub params: Vec<RustGenericParam>,
    pub r#where: Vec<RustWherePredicate>,
}

#[derive(Debug, Clone)]
pub(crate) enum RustGenericParam {
    Lifetime(RustLifetime),
    Type(String, Vec<RustTypeParamBound>, Option<RustType>),
    Const(String, RustType, Option<RustExpression>),
}

#[derive(Debug, Clone)]
pub(crate) enum RustTypeParamBound {
    Type(RustType),
    Lifetime(RustLifetime),
}

#[derive(Debug, Clone)]
pub(crate) enum RustWherePredicate {
    Type(RustType, Vec<RustTypeParamBound>),
    Lifetime(RustLifetime, Vec<RustLifetime>),
}
