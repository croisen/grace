#![allow(dead_code, unused)] // remove later
mod api;
mod core;
mod utils;

use crate::core::ast::sql::expression::SQLExpression;
use crate::core::ast::sql::{SQLAst, SQLParserState};

fn main() {
    let source = r#"
        SELECT
            age * 2 + 1 as new_age,
            lname as Last,
            fname as first,
        FROM test_table;
    "#;

    let output = SQLAst::new(source);
    for ast in output.root {
        println!("{ast:#?}");
    }

    println!();
    for err in output.errors {
        println!("{err:?}");
    }
}
