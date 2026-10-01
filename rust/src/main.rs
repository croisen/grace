mod api;
mod utils;

use crate::api::lexers::rust::RustToken;

fn main() {
    let source = r#"
        let x: i64 = 9_999;
    "#;

    let a = RustToken::tokenize(source).unwrap();
    println!("{a:#?}");
}
