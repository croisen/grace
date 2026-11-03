use crate::core::ast::rust::item::alias::RustTypeAlias;
use crate::core::ast::rust::item::r#const::RustConst;
use crate::core::ast::rust::item::r#enum::RustEnum;
use crate::core::ast::rust::item::r#extern::RustExtern;
use crate::core::ast::rust::item::function::RustFunction;
use crate::core::ast::rust::item::r#impl::RustImpl;
use crate::core::ast::rust::item::module::RustModule;
use crate::core::ast::rust::item::r#static::RustStatic;
use crate::core::ast::rust::item::r#struct::RustStruct;
use crate::core::ast::rust::item::r#trait::RustTrait;
use crate::core::ast::rust::item::r#use::RustUse;
use crate::core::lexers::rust::{RustToken, RustTokenKind};

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

#[derive(Debug, Clone)]
pub enum RustItem {
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
