use crate::api::GraceError;
use crate::api::ast::java::annotation::JavaAnnotationType;
use crate::api::ast::java::class::JavaClass;
use crate::api::ast::java::enumd::JavaEnum;
use crate::api::ast::java::interface::JavaInterface;
use crate::api::ast::java::packages::{JavaImport, JavaPackage};
use crate::api::ast::java::record::JavaRecord;
use crate::api::ast::java::structd::JavaStruct;
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
pub(crate) enum JavaAst {
    Class(JavaClass),
    Interface(JavaInterface),
    Struct(JavaStruct),
    Enum(JavaEnum),
    Record(JavaRecord),
    AnnotationType(JavaAnnotationType),
    Package(JavaPackage),
    Import(JavaImport),
}

impl JavaAst {
    pub fn new(
        tokens: Vec<JavaToken>,
        errors: Vec<GraceError>,
    ) -> (Vec<Self>, Vec<GraceError>) {
        (vec![], errors)
    }
}
