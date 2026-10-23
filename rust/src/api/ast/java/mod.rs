use crate::api::GraceError;
use crate::api::ast::java::item::JavaItem;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};
use crate::utils::prev_iter::PrevIter;

pub mod annotation;
pub mod class;
pub mod constructor;
pub mod enumd;
pub mod expression;
pub mod field;
pub mod generic_type;
pub mod interface;
pub mod item;
pub mod member;
pub mod method;
pub mod packages;
pub mod parameter;
pub mod record;
pub mod statement;
pub mod structd;
pub mod type_parameter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaRoot(Vec<JavaItem>);
