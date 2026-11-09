use crate::core::ast::java::annotation::JavaAnnotationType;
use crate::core::ast::java::class::JavaClass;
use crate::core::ast::java::enumd::JavaEnum;
use crate::core::ast::java::interface::JavaInterface;
use crate::core::ast::java::packages::{JavaImport, JavaPackage};
use crate::core::ast::java::record::JavaRecord;
use crate::core::ast::java::structd::JavaStruct;
use crate::core::lexers::java::{JavaToken, JavaTokenKind};
use crate::core::parser::output::ParserOutput;
use crate::core::parser::state::ParserState;

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

pub type JavaParserState<'a> = ParserState<'a, JavaTokenKind, JavaAst>;

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

impl JavaAst {
    pub fn new<'a>(source: &'a str) -> ParserOutput<Self> {
        let mut state = ParserState::<'a, JavaTokenKind, Self>::new(source);

        state.into()
    }
}
