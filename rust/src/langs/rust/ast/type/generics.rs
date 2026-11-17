use crate::langs::rust::ast::expression::RustExpression;
use crate::langs::rust::ast::r#type::RustType;
use crate::langs::rust::ast::r#type::lifetime::RustLifetime;

#[derive(Debug, Clone)]
pub struct RustGeneric {
    pub params: Vec<RustGenericParam>,
    pub r#where: Vec<RustWherePredicate>,
}

#[derive(Debug, Clone)]
pub enum RustGenericParam {
    Lifetime(RustLifetime),
    Type(String, Vec<RustTypeParamBound>, Option<RustType>),
    Const(String, RustType, Option<RustExpression>),
}

#[derive(Debug, Clone)]
pub enum RustTypeParamBound {
    Type(RustType),
    Lifetime(RustLifetime),
}

#[derive(Debug, Clone)]
pub enum RustWherePredicate {
    Type(RustType, Vec<RustTypeParamBound>),
    Lifetime(RustLifetime, Vec<RustLifetime>),
}
