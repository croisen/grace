mod api;
mod utils;

use crate::api::lexers::rust::RustToken;

fn main() {
    let source = r#"
        pub(super) use std::fmt::{Formatter, Display, Result};
        pub use std::sync::{atomic::AtomicPtr, Arc, mpsc::channel, mpmc::something::{yada_yada::Woo}};
        pub(self) use crate::api::ast::rust::RustAst;
        pub(crate) use crate::api::lexers::rust::RustToken;

        let mut x: Vec<(i32, i32)> = i * &*i;
        let mut r#pub = 21; // wow a keyword as an identifier???
        const i = 0xFFFF;   // error
    "#;

    let (tokens, errors) = RustToken::tokenize(source);
    for token in &tokens {
        println!("{token:?}");
    }

    println!();
    for error in errors {
        println!("{error:?}");
    }
}
