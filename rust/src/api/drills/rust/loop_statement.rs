use rand::RngExt;

use crate::api::drills::rust::variable::generate_variable;

#[derive(Debug, Clone)]
pub struct LoopStatementDrill {
    pub answer: String,
    pub conditions: String,
}

pub fn generate_loop_statement() -> LoopStatementDrill {
    let operators = ["==", "<=", ">=", "<", ">"];
    let operator = operators[rand::rng().random_range(0..operators.len())];
    let bound = rand::rng().random_range(0..=20);
    let var = generate_variable(1, 1);

    LoopStatementDrill {
        answer: format!(
            "{}\n\nloop {{\n    if {} {} {} {{\n        break;\n    }}\n    println!(\"pass\");\n}}",
            var.code, var.name, operator, bound,
        ),
        conditions: format!(
            "{}\n\n\
             - make a loop statement.\n\
             - inside the loop, compare {} and {}.\n\
             - use {} as the comparative operator.\n\
             - if it is true, break out of the loop.\n\
             - otherwise, print pass.",
            var.conditions, var.name, bound, operator,
        ),
    }
}
