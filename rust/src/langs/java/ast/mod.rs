use crate::core::parser::state::ParserState;
use crate::langs::java::lexer::JavaTokenKind;

pub mod annotation;
pub mod class;
pub mod constructor;
pub mod r#enum;
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
pub mod r#struct;
pub mod type_parameter;

pub type JavaParserState<'a> = ParserState<'a, JavaTokenKind, JavaAst>;

pub enum JavaAst {}
