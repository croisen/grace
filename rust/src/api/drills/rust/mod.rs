mod if_statement;
mod variable;

pub use if_statement::{generate_if_statement, IfStatementDrill};
pub use variable::{generate_variable, VariableDrill};

use rand::RngExt;

fn callsign() -> String {
    let mut rng = rand::rng();

    let names = [
        "alpha", "bravo", "charlie", "delta", "echo",
        "foxtrot", "golf", "hotel", "india", "juliett",
        "kilo", "lima", "mike", "november", "oscar",
        "papa", "quebec", "romeo", "sierra", "tango",
        "uniform", "victor", "whiskey", "xray", "yankee",
        "zulu",
    ];

    names[rng.random_range(0..names.len())].to_string()
}
