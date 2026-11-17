use crate::core::parser::output::ParserOutput;
use crate::core::parser::state::ParserState;
use crate::langs::rust::ast::item::RustItem;
use crate::langs::rust::ast::visibility::RustVisibility;
use crate::langs::rust::lexer::RustTokenKind;

pub mod expression;
pub mod item;
pub mod pattern;
pub mod statement;
pub mod r#type;
pub mod visibility;

pub type RustParserState<'a> = ParserState<'a, RustTokenKind, RustAst>;

#[derive(Debug, Clone)]
pub struct RustAst {
    pub visibility: RustVisibility,
    pub item: RustItem,
}

impl RustAst {
    pub fn new<'a>(source: &'a str) -> ParserOutput<Self> {
        let mut state = ParserState::<'a, RustTokenKind, Self>::new(source);

        state.into()
    }
}
