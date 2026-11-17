// #![allow(dead_code, unused)] // remove later
mod api;
mod core;
mod langs;
mod utils;

use crate::api::drills::java::java_if_statement;
use crate::langs::sql::ast::SQLAst;

fn main() {
    let source = r#"
        SELECT
            age * 2 + 1 as new_age,
            lname as Last,
            fname as first,
        FRUM test_table;
    "#;

    let output = SQLAst::new(source);
    for ast in output.root {
        println!("{ast:#?}");
    }

    println!();
    for err in output.errors {
        println!("{err:?}");
    }

    java_if_statement();
}
