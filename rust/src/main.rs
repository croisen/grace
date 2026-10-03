mod api;
mod utils;

use crate::api::lexers::rust::RustToken;

fn main() {
    let source = r#"
        pub fn square(i: i32) -> i32 {
            i * i
        }

        }{
    "#;

    let a = RustToken::tokenize(source);
    match a {
        Ok(a) => println!("{a:#?}"),
        Err(a) => println!("{a:#?}"),
    }
}
