use crate::core::errors::GraceError;
use crate::core::parser::state::ParserState;

#[flutter_rust_bridge::frb(ignore)]
pub struct ParserOutput<L> {
    pub root: Vec<L>,
    pub errors: Vec<GraceError>,
}

impl<'a, TK, L> From<ParserState<'a, TK, L>> for ParserOutput<L> {
    fn from(value: ParserState<'a, TK, L>) -> Self {
        Self { root: value.root, errors: value.errors }
    }
}
