use rand::RngExt;

use crate::api::drills::rust::variable::generate_variable;

#[derive(Debug, Clone)]
pub struct WhileStatementDrill {
    pub answer: String,
    pub conditions: String,
}

pub fn generate_while_statement() -> WhileStatementDrill {
    let operators = ["==", "<=", ">=", "<", ">"];
    let operator = operators[rand::rng().random_range(0..operators.len())];
    let bound = rand::rng().random_range(0..=20);
    let var = generate_variable(1, 0);

    WhileStatementDrill {
        answer: format!(
            "{}\n\nwhile {} {} {} {{\n    println!(\"pass\");\n}}",
            var.code, var.name, operator, bound,
        ),
        conditions: format!(
            "{}\n\n\
             - make a while loop.\n\
             - make a while loop that compares {} and {}.\n\
             - use {} as the comparative operator.\n\
             - make it so that while it is true, it would print pass.",
            var.conditions, var.name, bound, operator,
        ),
    }
}
