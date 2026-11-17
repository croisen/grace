use crate::langs::rust::ast::item::alias::RustTypeAlias;
use crate::langs::rust::ast::item::r#const::RustConst;
use crate::langs::rust::ast::item::r#enum::RustEnum;
use crate::langs::rust::ast::item::r#extern::RustExtern;
use crate::langs::rust::ast::item::function::RustFunction;
use crate::langs::rust::ast::item::r#impl::RustImpl;
use crate::langs::rust::ast::item::module::RustModule;
use crate::langs::rust::ast::item::r#static::RustStatic;
use crate::langs::rust::ast::item::r#struct::RustStruct;
use crate::langs::rust::ast::item::r#trait::RustTrait;
use crate::langs::rust::ast::item::r#use::RustUse;

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
