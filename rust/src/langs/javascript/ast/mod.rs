use crate::core::parser::state::ParserState;
use crate::langs::javascript::lexer::JavaScriptTokenKind;

pub type JavaScriptParserState<'a> =
    ParserState<'a, JavaScriptTokenKind, JavaScriptAst>;

pub enum JavaScriptAst {}
