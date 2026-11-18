use crate::core::parser::state::ParserState;
use crate::langs::cpp::lexer::CppTokenKind;

pub type CppParserState<'a> = ParserState<'a, CppTokenKind, CppAst>;

pub enum CppAst {}
