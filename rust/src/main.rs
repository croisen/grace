#![allow(dead_code, unused)] // remove later
mod api;
mod core;
mod utils;

use crate::core::ast::sql::SQLAst;

fn main() {
    let source = r#"
        CREATE DATABASE IF NOT EXISTS test_db1;
        CREATE DATABASE IF EXISTS test_db2;
        CREATE DATABASE test_db3;

        CREATE TABLE IF NOT EXISTS test_table(
            id INT AUTO INCREMENT PRIMARY KEY,
            fname TEXT,
            lname TEXT
        );

        CREATE UNIQUE INDEX IF NOT EXISTS test_idx1 on test_table (
            fname,
            lname
        );

        CREATE INDEX test_idx2 on test_table (
            id,
            fname
        );
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
