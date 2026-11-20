use rand::RngExt;

use crate::api::rust::callsign;

#[derive(Debug, Clone)]
pub struct VariableDrill {
    pub code: String,
    pub conditions: String,
    pub name: String,
    pub state: String,
    pub var_type: String,
    pub value: String,
}

pub fn generate_variable(choice: usize, if_mut: usize) -> VariableDrill {
    let variable_types = ["String", "i32"];

    let var_type = match choice {
        0 => variable_types[0],
        1 => variable_types[1],
        _ => variable_types[0],
    };

    let name = callsign();

    let value = match choice {
        0 => callsign(),
        1 => rand::rng().random_range(0..=257).to_string(),
        _ => callsign(),
    };

    let (is_mut, state) = match if_mut {
        1 => ("must be mutable.", " mut"),
        0 => ("must not be mutable.", ""),
        _ => ("may or may not be mutable.", ""),
    };

    // Fixed: String values now produce valid Rust
    let var_value = if var_type == "String" {
        format!("String::from({:?})", value)
    } else {
        value.clone()
    };

    VariableDrill {
        code: format!("let{} {}: {} = {};", state, name, var_type, var_value),
        conditions: format!(
            "- variable must be named \"{}\"\n\
             - variable \"{}\" must be of type \"{}\"\n\
             - variable \"{}\" must have a value of \"{}\"\n\
             - variable \"{}\" {}",
            name, name, var_type, name, value, name, is_mut
        ),
        name,
        state: state.to_string(),
        var_type: var_type.to_string(),
        value,
    }
}

pub fn generate_distinct_variables(
    count: usize,
    choice: usize,
    if_mut: usize,
) -> Vec<VariableDrill> {
    assert!(count <= 26, "only 26 callsigns exist");

    let mut vars: Vec<VariableDrill> = Vec::with_capacity(count);

    while vars.len() < count {
        let next = generate_variable(choice, if_mut);

        if vars.iter().any(|var| var.name == next.name) {
            continue;
        }

        vars.push(next);
    }

    vars
}
