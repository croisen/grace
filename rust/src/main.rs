mod api;
mod utils;

use crate::api::ast::sql::SqlAst;
use crate::api::lexers::sql::SQLToken;

fn main() {
    let source = r#"
        create    database test_db;
    "#;

    let (tokens, errors) = SQLToken::tokenize(source);
    for token in &tokens {
        println!("{token:?}");
    }

    println!();
    let (ast, errors) = SqlAst::new(tokens, errors);
    for ast in &ast {
        println!("{ast:?}");
    }

    println!();
    for error in errors {
        println!("{error:?}");
    }
}
