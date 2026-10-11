#![feature(allocator_api, ptr_cast_slice, sized_type_properties)]

mod api;
mod utils;

use crate::api::ast::rust::RustAst;
use crate::api::lexers::rust::RustToken;

fn main() {
    let source = r#"
        pub use std::path::{Path, PathBuf};

        use crate::api::lexers::{GraceError, LexingError};
        use crate::utils::prev_iter::{PrevIter, PrevIterator}

        pub fn square(i: i32) -> i32 {
            i * i
        }

        }{
    "#;

    let (tokens, errors) = RustToken::tokenize(source);
    for token in &tokens {
        println!("\t{token:?}");
    }
    let (ast, errors) = RustAst::new(tokens, errors);
    // println!("{ast:#?}");
    for error in errors {
        println!("{error}");
    }
}
