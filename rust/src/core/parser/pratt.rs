use std::fmt::Debug;

use logos::Logos;

use crate::core::errors::kind::GraceErrorKind;
use crate::core::lexers::{LexerExtras, Token};
use crate::core::parser::state::ParserState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrattAssociativity {
    Left(isize),
    Right(isize),
    None(isize),
}

impl PrattAssociativity {
    pub fn power(&self) -> (isize, isize) {
        match self {
            Self::Left(i) => (i * 2, i * 2 + 1),
            Self::Right(i) => (i * 2 + 1, i * 2),
            Self::None(i) => (i * 2, i * 2 + 1),
        }
    }

    pub fn left_power(&self) -> isize {
        self.power().0
    }

    pub fn right_power(&self) -> isize {
        self.power().1
    }
}

pub trait PrattLanguage<'a> {
    type Root;
    type TokenKind: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Clone
        + PartialEq
        + Debug;

    fn prefix_binding(
        token: &Token<'a, Self::TokenKind>,
    ) -> Option<PrattAssociativity>;
    fn infix_binding(
        token: &Token<'a, Self::TokenKind>,
    ) -> Option<PrattAssociativity>;
    fn postfix_binding(
        token: &Token<'a, Self::TokenKind>,
    ) -> Option<PrattAssociativity>;

    fn parse_atom(
        state: &mut ParserState<'a, Self::TokenKind, Self::Root>,
    ) -> Self;
    fn parse_prefix(
        state: &mut ParserState<'a, Self::TokenKind, Self::Root>,
        rhs: Self,
    ) -> Self;
    fn parse_postfix(
        state: &mut ParserState<'a, Self::TokenKind, Self::Root>,
        lhs: Self,
    ) -> Self;
    fn parse_infix(
        state: &mut ParserState<'a, Self::TokenKind, Self::Root>,
        lhs: Self,
        rhs: Self,
    ) -> Self;
}

pub fn parse_expr<'a, L>(
    state: &mut ParserState<'a, L::TokenKind, L::Root>,
    min: isize,
) -> L
where
    L: PrattLanguage<'a>,
    L::TokenKind: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Clone
        + PartialEq
        + Debug,
{
    let mut lhs = parse_initial(state);
    loop {
        let Some(token) = state.token_current() else {
            break;
        };

        if let Some(binding) = L::postfix_binding(token) {
            let left = binding.left_power();
            if left < min {
                break;
            }

            state.advance();
            lhs = L::parse_postfix(state, lhs);
            continue;
        }

        if let Some(binding) = L::infix_binding(token) {
            let (left, right) = binding.power();
            if left < min {
                break;
            }

            state.advance();
            let rhs = parse_expr(state, right);
            lhs = L::parse_infix(state, lhs, rhs);
            continue;
        }
    }

    lhs
}

fn parse_initial<'a, L>(state: &mut ParserState<'a, L::TokenKind, L::Root>) -> L
where
    L: PrattLanguage<'a>,
    L::TokenKind: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Clone
        + PartialEq
        + Debug,
{
    let Ok(token) = state.get_token() else {
        return L::parse_atom(state);
    };

    if let Some(binding) = L::prefix_binding(token) {
        state.advance();
        let rhs = parse_expr(state, binding.right_power());
        return L::parse_prefix(state, rhs);
    }

    L::parse_atom(state)
}
