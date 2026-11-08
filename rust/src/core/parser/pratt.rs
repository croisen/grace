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

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None(_))
    }
}

pub trait PrattLanguage<'a>
where
    Self: Sized,
{
    type Root;
    type TokenKind: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Clone
        + PartialEq
        + Debug;

    fn prefix_binding(
        token: &Token<'a, Self::TokenKind>,
    ) -> Option<PrattAssociativity> {
        todo!("");
    }
    fn infix_binding(
        token: &Token<'a, Self::TokenKind>,
    ) -> Option<PrattAssociativity> {
        todo!("");
    }
    fn postfix_binding(
        token: &Token<'a, Self::TokenKind>,
    ) -> Option<PrattAssociativity> {
        todo!("");
    }

    fn parse_atom(
        state: &mut ParserState<'a, Self::TokenKind, Self::Root>,
    ) -> Option<Self> {
        todo!("");
    }
    fn parse_prefix(
        state: &mut ParserState<'a, Self::TokenKind, Self::Root>,
        op: String,
        rhs: Self,
    ) -> Option<Self> {
        todo!("");
    }
    fn parse_postfix(
        state: &mut ParserState<'a, Self::TokenKind, Self::Root>,
        op: String,
        lhs: Self,
    ) -> Option<Self> {
        todo!("");
    }
    fn parse_infix(
        state: &mut ParserState<'a, Self::TokenKind, Self::Root>,
        op: String,
        lhs: Self,
        rhs: Self,
    ) -> Option<Self> {
        todo!("");
    }
}

pub fn parse_expr<'a, L>(
    state: &mut ParserState<'a, L::TokenKind, L::Root>,
    min: isize,
) -> Option<L>
where
    L: PrattLanguage<'a>,
    L::TokenKind: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Clone
        + PartialEq
        + Debug,
{
    let mut lhs = parse_initial(state)?;
    let mut non_assoc = None;
    loop {
        let Some(token) = state.token_current() else {
            break;
        };

        if let Some(binding) = L::postfix_binding(token) {
            let left = binding.left_power();
            if left < min {
                break;
            }

            let text = token.text.to_string();
            state.advance();
            lhs = L::parse_postfix(state, text, lhs)?;
            continue;
        }

        if let Some(binding) = L::infix_binding(token) {
            let (left, right) = binding.power();
            if left < min {
                break;
            }

            if binding.is_none() {
                if non_assoc == Some(left) {
                    state.record_syntax_error(
                        format!(
                            "Same none associative operator `{}` used in succession",
                            token.text,
                        )
                    );
                    return None;
                }

                non_assoc = Some(left);
            }

            let text = token.text.to_string();
            state.advance();
            let rhs = parse_expr(state, right)?;
            lhs = L::parse_infix(state, text, lhs, rhs)?;
            continue;
        }
    }

    Some(lhs)
}

fn parse_initial<'a, L>(
    state: &mut ParserState<'a, L::TokenKind, L::Root>,
) -> Option<L>
where
    L: PrattLanguage<'a>,
    L::TokenKind: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Clone
        + PartialEq
        + Debug,
{
    let token = state.get_token()?;
    if let Some(binding) = L::prefix_binding(token) {
        let text = token.text.to_string();
        state.advance();
        let rhs = parse_expr(state, binding.right_power())?;
        return L::parse_prefix(state, text, rhs);
    }

    L::parse_atom(state)
}
