use crate::core::ast::java::annotation::JavaAnnotationType;
use crate::core::ast::java::class::JavaClass;
use crate::core::ast::java::enumd::JavaEnum;
use crate::core::ast::java::interface::JavaInterface;
use crate::core::ast::java::packages::{JavaImport, JavaPackage};
use crate::core::ast::java::record::JavaRecord;
use crate::core::ast::java::structd::JavaStruct;
use crate::core::lexers::java::{JavaToken, JavaTokenKind};

pub mod annotation;
pub mod class;
pub mod constructor;
pub mod enumd;
pub mod expression;
pub mod field;
pub mod generic_type;
pub mod interface;
pub mod member;
pub mod method;
pub mod packages;
pub mod parameter;
pub mod record;
pub mod statement;
pub mod structd;
pub mod type_parameter;

#[derive(Debug, Clone)]
pub enum JavaAst {
    Class(JavaClass),
    Interface(JavaInterface),
    Struct(JavaStruct),
    Enum(JavaEnum),
    Record(JavaRecord),
    AnnotationType(JavaAnnotationType),
    Package(JavaPackage),
    Import(JavaImport),
}
