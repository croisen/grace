use flutter_rust_bridge::frb;
use rand::RngExt;

use crate::api::rust::variable::generate_variable;

#[derive(Debug, Clone)]
pub struct IfStatementDrill {
    pub answer: String,
    pub conditions: String,
}

#[frb(sync)]
pub fn generate_if_statement() -> IfStatementDrill {
    let operators = ["==", "<=", ">=", "<", ">"];
    let operator_index = rand::rng().random_range(0..operators.len());
    let operator = operators[operator_index];

    let var_one = generate_variable(1, 0);
    let mut var_two = generate_variable(1, 0);

    while var_one.name == var_two.name {
        var_two = generate_variable(1, 0);
    }

    IfStatementDrill {
        answer: format!(
            "{}\n{}\n\nif {} {} {} {{\n    println!(\"pass\");\n}} else {{\n    println!(\"fail\");\n}}",
            var_one.code, var_two.code, var_one.name, operator, var_two.name,
        ),
        conditions: format!(
            "{}\n\n{}\n\n\
             - make an if statement.\n\
             - make an if statement that compares {} and {}.\n\
             - use {} as the comparative operator.\n\
             - make it so that if it is true, it would print pass, and if not, it would print fail.",
            var_one.conditions,
            var_two.conditions,
            var_one.name,
            var_two.name,
            operator,
        ),
    }
}
