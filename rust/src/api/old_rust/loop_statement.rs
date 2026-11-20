use flutter_rust_bridge::frb;
use rand::RngExt;

use crate::api::rust::variable::generate_variable;

#[derive(Debug, Clone)]
pub struct LoopStatementDrill {
    pub answer: String,
    pub conditions: String,
}
#[frb(sync)]
pub fn generate_loop_statement() -> LoopStatementDrill {
    let bound = rand::rng().random_range(3..=8);
    // Force mutable so we can increment
    let var = generate_variable(1, 1);

    LoopStatementDrill {
        answer: format!(
            "{}\n\nloop {{\n    if {} >= {} {{\n        break;\n    }}\n    println!(\"pass\");\n    {} += 1;\n}}",
            var.code, var.name, bound, var.name
        ),
        conditions: format!(
            "{}\n\n\
             - make a loop statement.\n\
             - inside the loop, if {} is greater than or equal to {}, break.\n\
             - otherwise print pass and increment {} by 1.",
            var.conditions, var.name, bound, var.name
        ),
    }
}
