use crate::api::GraceError;
use crate::api::ast::rust::item::alias::RustTypeAlias;
use crate::api::ast::rust::item::r#const::RustConst;
use crate::api::ast::rust::item::r#enum::RustEnum;
use crate::api::ast::rust::item::r#extern::RustExtern;
use crate::api::ast::rust::item::function::RustFunction;
use crate::api::ast::rust::item::r#impl::RustImpl;
use crate::api::ast::rust::item::module::RustModule;
use crate::api::ast::rust::item::r#static::RustStatic;
use crate::api::ast::rust::item::r#struct::RustStruct;
use crate::api::ast::rust::item::r#trait::RustTrait;
use crate::api::ast::rust::item::r#use::RustUse;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

pub mod alias;
pub mod r#const;
pub mod r#enum;
pub mod r#extern;
pub mod function;
pub mod r#impl;
pub mod module;
pub mod r#static;
pub mod r#struct;
pub mod r#trait;
pub mod r#use;

pub(crate) enum RustItem {
    Function(RustFunction),
    Struct(RustStruct),
    Enum(RustEnum),
    Module(RustModule),
    Use(RustUse),
    Trait(RustTrait),
    Impl(RustImpl),
    TypeAlias(RustTypeAlias),
    Const(RustConst),
    Static(RustStatic),
    Extern(RustExtern),
}
