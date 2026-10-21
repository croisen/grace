use crate::api::GraceError;
use crate::api::lexers::LexingError;

pub mod java;
pub mod rust;
pub mod sql;

// For precedence
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AstOperation {
    Literal,      // Ident
    FunctionCall, // a(), { }, [1, ...], {struct}.member
    UnaryOp,      // negate (!), ref (&), deref (*), ++, --
    MulOp,        // *, /, %
    AddOp,        // +, -
    BitShft,      // <<, >>

    // Well the creator of C was thinking Bitwise operators should be above comparisons
    CompOp, // <, <=, >=, >
    CompEq, // ==, !=

    // but it was way after C became the standard sooo, we following C
    BitAnd,
    BitXor,
    BitOr,

    LogAnd,
    LogOr,
    Ternary,    // We don't have this in rust but java has it
    Assignment, // We don't use this technically?
    Comma,
}
