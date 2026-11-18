use crate::core::parser::output::ParserOutput;
use crate::core::parser::state::ParserState;
use crate::langs::c::ast::declaration::CDecl;
use crate::langs::c::ast::declaration::function::CFunctionDecl;
use crate::langs::c::lexer::CTokenKind;

pub mod declaration;
pub mod declarator;
pub mod expression;
pub mod statement;
pub mod r#type;

pub type CParserState<'a> = ParserState<'a, CTokenKind, CAst>;

#[derive(Debug, Clone)]
pub enum CAst {
    Function(CFunctionDecl),
    Declaration(CDecl),
}

impl CAst {
    pub fn new<'a>(source: &'a str) -> ParserOutput<Self> {
        let mut state = ParserState::<'a, CTokenKind, Self>::new(source);

        state.into()
    }
}
