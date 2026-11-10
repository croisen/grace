#![allow(dead_code, unused)] // remove later
mod api;
mod config;
mod core;
mod langs;
mod utils;

use crate::api::drills::java::java_if_statement;
use crate::api::fsrs::{Concept, FSRS, Grade};
use crate::core::ast::sql::expression::SQLExpression;
use crate::core::ast::sql::{SQLAst, SQLParserState};
use crate::core::edit::edit_distance_sen;

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

    java_if_statement();

    println!("Edit distance: {}", edit_distance_sen("kro1sen", "croisen"));

    let mut fsrs = FSRS::new(Concept::ForLoop);
    let grades = &[Grade::Again, Grade::Hard, Grade::Good, Grade::Easy];
    for i in 0..90 {
        let grade = grades[rand::random_range(0..grades.len())];
        fsrs.review(grade);
        println!(
            "Stability: {:>4.2} Difficulty: {:>4.2} Retain: {:>3.2} Grade: {grade:?}",
            fsrs.stability,
            fsrs.difficulty,
            fsrs.retainability()
        );
    }
}
