use crate::api::GraceError;
use crate::api::ast::rust::visitem::VisItem;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone, PartialEq)]
#[flutter_rust_bridge::frb(ignore)]
pub enum Pattern {
    Literal(String),           // 1, 2, "Raw string"
    Identifier(String),        // names that aren't keywords
    Wildcard,                  // _
    Rest,                      // .. [see how it is used in tuple]
    Reference(Box<Pattern>),   // &a, &mut a, &&a, &&mut a
    Struct,                    // A {a: b, ...}, A {0: pattern, 1: pattern}
    TupleStruct(Vec<Pattern>), // A(a, b) no rest pattern here
    Tuple(Vec<Pattern>),       // (a, b, ..) <=== here, (a,), (a,b,)
    Grouped(Box<Pattern>),     // (pattern)
    Slice(Box<Pattern>),       // [pattern (, pattern)*],
    Path,                      // a::b()
}
