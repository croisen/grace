use std::fmt::Debug;
use std::ops::Deref;

use logos::Logos;

use crate::core::errors::kind::GraceErrorKind;
use crate::core::lexers::{LexerExtras, Token};
use crate::core::parser::state::ParserState;

#[derive(Debug, Clone)]
pub struct Token2 {
    pub text: String,
    pub line: usize,
    pub col: usize,
}

impl Default for Token2 {
    fn default() -> Self {
        Self { text: "Placeholder".into(), line: 0, col: 0 }
    }
}

impl Deref for Token2 {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl<'a, TK> From<&Token<'a, TK>> for Token2
where
    TK: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Clone
        + PartialEq
        + Debug,
{
    fn from(value: &Token<'a, TK>) -> Self {
        Self { text: value.text.to_string(), line: value.line, col: value.col }
    }
}

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
    ) -> Option<PrattAssociativity>;
    fn infix_binding(
        token: &Token<'a, Self::TokenKind>,
    ) -> Option<PrattAssociativity>;
    fn postfix_binding(
        token: &Token<'a, Self::TokenKind>,
    ) -> Option<PrattAssociativity>;

    fn parse_atom(
        state: &mut ParserState<'a, Self::TokenKind, Self::Root>,
    ) -> Option<Self>;
    fn parse_prefix(
        state: &mut ParserState<'a, Self::TokenKind, Self::Root>,
        min_bp: isize,
        op: Token2,
    ) -> Option<Self>;
    fn parse_postfix(
        state: &mut ParserState<'a, Self::TokenKind, Self::Root>,
        op: Token2,
        lhs: Self,
    ) -> Option<Self>;
    fn parse_infix(
        state: &mut ParserState<'a, Self::TokenKind, Self::Root>,
        min_bp: isize,
        op: Token2,
        lhs: Self,
    ) -> Option<Self>;
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
    state.advance();
    while state.not_at_end() {
        let Some(token) = state.token_current() else {
            break;
        };

        if let Some(binding) = L::postfix_binding(token) {
            let left = binding.left_power();
            if left < min {
                break;
            }

            let token = token.into();
            state.advance();
            lhs = L::parse_postfix(state, token, lhs)?;
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
                        "Non-associative operators can't be chained",
                    );

                    return None;
                }

                non_assoc = Some(left);
            }

            let token = token.into();
            state.advance();
            lhs = L::parse_infix(state, right, token, lhs)?;
            continue;
        }

        break;
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
        let token = token.into();
        state.advance();
        return L::parse_prefix(state, binding.right_power(), token);
    }

    L::parse_atom(state)
}
