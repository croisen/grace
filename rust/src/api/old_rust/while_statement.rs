use flutter_rust_bridge::frb;
use rand::RngExt;

use crate::api::old_rust::variable::generate_variable;

#[derive(Debug, Clone)]
pub struct WhileStatementDrill {
    pub answer: String,
    pub conditions: String,
}
#[frb(sync)]
pub fn generate_while_statement() -> WhileStatementDrill {
    let bound = rand::rng().random_range(3..=8);
    // Force mutable so we can increment
    let var = generate_variable(1, 1);

    WhileStatementDrill {
        answer: format!(
            "{}\n\nwhile {} < {} {{\n    println!(\"pass\");\n    {} += 1;\n}}",
            var.code, var.name, bound, var.name
        ),
        conditions: format!(
            "{}\n\n\
             - make a while loop.\n\
             - the loop should continue while {} is less than {}.\n\
             - inside the loop, print pass and then increment {} by 1.",
            var.conditions, var.name, bound, var.name
        ),
    }
}
