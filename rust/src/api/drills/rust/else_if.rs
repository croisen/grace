use rand::RngExt;
use flutter_rust_bridge::frb;
use crate::api::drills::rust::variable::generate_distinct_variables;

#[derive(Debug, Clone)]
pub struct ElseIfDrill {
    pub answer: String,
    pub conditions: String,
}
#[frb(sync)]
pub fn generate_else_if() -> ElseIfDrill {
    let operators = ["==", "<=", ">=", "<", ">"];
    let op_one = operators[rand::rng().random_range(0..operators.len())];
    let op_two = operators[rand::rng().random_range(0..operators.len())];

    let vars = generate_distinct_variables(3, 1, 0);
    let first = &vars[0];
    let second = &vars[1];
    let third = &vars[2];

    ElseIfDrill {
        answer: format!(
            "{}\n{}\n{}\n\nif {} {} {} {{\n    println!(\"pass\");\n}} else if {} {} {} {{\n    println!(\"retry\");\n}} else {{\n    println!(\"fail\");\n}}",
            first.code,
            second.code,
            third.code,
            first.name,
            op_one,
            second.name,
            first.name,
            op_two,
            third.name,
        ),
        conditions: format!(
            "{}\n\n{}\n\n{}\n\n\
             - make an if statement that compares {} and {}.\n\
             - use {} as the comparative operator.\n\
             - make it so that if it is true, it would print pass.\n\
             - make an else if statement that compares {} and {}.\n\
             - use {} as the comparative operator.\n\
             - make it so that if it is true, it would print retry.\n\
             - make an else statement that prints fail.",
            first.conditions,
            second.conditions,
            third.conditions,
            first.name,
            second.name,
            op_one,
            first.name,
            third.name,
            op_two,
        ),
    }
}
