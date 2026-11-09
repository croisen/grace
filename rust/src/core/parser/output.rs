use std::collections::HashSet;
use std::fmt::Debug;

use logos::Logos;

use crate::core::errors::GraceError;
use crate::core::errors::kind::GraceErrorKind;
use crate::core::lexers::LexerExtras;
use crate::core::parser::state::ParserState;

pub struct ParserOutput<L> {
    pub root: Vec<L>,
    pub errors: Vec<GraceError>,
}

impl<'a, TK, L> From<ParserState<'a, TK, L>> for ParserOutput<L>
where
    TK: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Clone
        + PartialEq
        + Debug,
{
    fn from(value: ParserState<'a, TK, L>) -> Self {
        Self {
            root: value.root,
            errors: value
                .errors
                .into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect(),
        }
    }
}
