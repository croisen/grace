use rand::RngExt;

use crate::api::drills::rust::variable::generate_variable;

#[derive(Debug, Clone)]
pub struct MatchStatementDrill {
    pub answer: String,
    pub conditions: String,
}

pub fn generate_match_statement() -> MatchStatementDrill {
    let mut literals = Vec::new();

    while literals.len() < 2 {
        let literal = rand::rng().random_range(0..=9);

        if literals.contains(&literal) {
            continue;
        }

        literals.push(literal);
    }

    literals.sort();

    let var = generate_variable(1, 0);

    MatchStatementDrill {
        answer: format!(
            "{}\n\nmatch {} {{\n    {} => println!(\"pass\"),\n    {} => println!(\"retry\"),\n    _ => println!(\"fail\"),\n}}",
            var.code, var.name, literals[0], literals[1],
        ),
        conditions: format!(
            "{}\n\n\
             - make a match statement.\n\
             - match on {}.\n\
             - if it matches {}, print pass.\n\
             - if it matches {}, print retry.\n\
             - for anything else, print fail.",
            var.conditions, var.name, literals[0], literals[1],
        ),
    }
}
