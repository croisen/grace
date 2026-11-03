use std::fmt::Debug;

use logos::Logos;

use crate::core::errors::kind::GraceErrorKind;
use crate::core::lexers::LexerExtras;

pub enum PrattAssociativity {
    Left,
    Right,
    None,
}

pub struct PrattOperator {
    pub precedence: usize,
    pub associativity: PrattAssociativity,
}

impl PrattOperator {
    pub fn left(precedence: usize) -> Self {
        Self { precedence, associativity: PrattAssociativity::Left }
    }

    pub fn right(precedence: usize) -> Self {
        Self { precedence, associativity: PrattAssociativity::Right }
    }

    pub fn none(precedence: usize) -> Self {
        Self { precedence, associativity: PrattAssociativity::None }
    }
}

#[flutter_rust_bridge::frb(ignore)]
pub trait ParserPratt<'a, TK, LangExpression>
where
    TK: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Debug,
{
}
