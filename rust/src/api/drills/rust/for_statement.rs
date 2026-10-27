use rand::RngExt;

use crate::api::drills::rust::distinct_callsigns;

#[derive(Debug, Clone)]
pub struct ForStatementDrill {
    pub answer: String,
    pub conditions: String,
}

pub fn generate_for_statement() -> ForStatementDrill {
    let name = distinct_callsigns(1).remove(0);
    let start = rand::rng().random_range(0..=5);
    let end = rand::rng().random_range((start + 1)..=(start + 10));

    ForStatementDrill {
        answer: format!(
            "for {} in {}..{} {{\n    println!(\"pass\");\n}}",
            name, start, end,
        ),
        conditions: format!(
            "- make a for loop.\n\
             - the loop variable must be named \"{}\".\n\
             - iterate from {} up to but not including {} using .. .\n\
             - inside the loop, print pass.",
            name, start, end,
        ),
    }
}
