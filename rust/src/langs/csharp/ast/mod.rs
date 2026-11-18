use crate::core::parser::state::ParserState;
use crate::langs::csharp::lexer::CSharpTokenKind;

pub type CSharpParserState<'a> = ParserState<'a, CSharpTokenKind, CSharpAst>;

pub enum CSharpAst {}
