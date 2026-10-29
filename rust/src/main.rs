#![allow(dead_code, unused)] // gotta remove these later
mod api;
mod utils;

use crate::api::ast::sql::SqlAst;

fn main() {
    let source = r#"
        create database test_db;
        create table if not exists ze_table(
            id int auto increment not null primary key,
            age int not null,
            name text not null
        );
    "#;

    println!();
    let (ast, errors) = SqlAst::new(source);
    for ast in &ast {
        println!("{ast:#?}");
    }

    println!();
    for error in errors {
        println!("{error:?}");
    }
}
