use crate::core::ast::rust::item::RustItem;
use crate::core::ast::rust::visibility::RustVisibility;
use crate::core::lexers::rust::RustTokenKind;
use crate::core::parser::output::ParserOutput;
use crate::core::parser::state::ParserState;

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
