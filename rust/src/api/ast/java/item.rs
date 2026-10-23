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

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub enum JavaItem {
    Class(JavaClass),
    Interface(JavaInterface),
    Struct(JavaStruct),
    Enum(JavaEnum),
    Record(JavaRecord),
    AnnotationType(JavaAnnotationType),
    Package(JavaPackage),
    Import(JavaImport),
    Unknown,
}

impl JavaItem {
    pub fn new_main_parse_body(source: String) -> Self {
        Self::Unknown
    }
}
