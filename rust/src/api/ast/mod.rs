use logos::Logos;

use crate::api::GraceError;
use crate::api::lexers::{LexerExtras, LexingError, Token};
use crate::utils::prev_iter::PrevIter;

// Definition of representations of code in struct and enums are from oak.rs examples
// With the actual parsing done manually, there are some differences since I see things a bit
// differently
pub mod java;
pub mod rust;
pub mod sql;

fn unexpected_eof<'a, TK>(
    iter: &mut PrevIter<'_, Token<'a, TK>>,
    errors: &mut Vec<GraceError>,
) where
    TK: Logos<'a, Source = str, Error = LexingError, Extras = LexerExtras>
        + Default,
{
    iter.seek_end();
    if let Some(t) = iter.prev() {
        GraceError::syntax_error(errors, t, "Unexpected EOF");
        let _ = iter.next();
        return;
    }

    GraceError::syntax_error(
        errors,
        &Token::<'_, TK>::default(),
        "Unexpected EOF",
    );
}
