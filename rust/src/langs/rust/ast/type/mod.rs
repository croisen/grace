use crate::langs::rust::ast::expression::RustExpression;
use crate::langs::rust::ast::r#type::lifetime::RustLifetime;

pub mod generics;
pub mod lifetime;
pub mod parameter;
pub mod type_parameter;

#[derive(Debug, Clone)]
pub enum RustType {
    Path(Vec<String>),
    Reference(Option<RustLifetime>, bool, Box<RustType>),
    Tuple(Vec<RustType>),
    Array(Box<RustType>, RustExpression),
    Slice(Box<RustType>),
    Fn(Vec<RustType>, Option<Box<RustType>>),
    Inferred,
}
