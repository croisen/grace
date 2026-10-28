use rand::RngExt;
use flutter_rust_bridge::frb;
use crate::api::drills::rust::distinct_callsigns;

#[derive(Debug, Clone)]
pub struct FunctionDrill {
    pub answer: String,
    pub conditions: String,
}

#[frb(sync)]
pub fn generate_function() -> FunctionDrill {
    match rand::rng().random_range(0..4) {
        0 => function_no_params(),
        1 => function_one_param_return(),
        2 => function_two_param_return(),
        _ => function_string_param(),
    }
}

fn function_no_params() -> FunctionDrill {
    let name = distinct_callsigns(1).remove(0);

    FunctionDrill {
        answer: format!("fn {name}() {{\n    println!(\"pass\");\n}}"),
        conditions: format!(
            "- make a function.\n\
             - function must be named \"{name}\".\n\
             - function must not take any parameters.\n\
             - function must not return a value.\n\
             - inside the function, print pass."
        ),
    }
}

fn function_one_param_return() -> FunctionDrill {
    let names = distinct_callsigns(2);
    let name = &names[0];
    let param = &names[1];

    // Idiomatic: expression-style return (no explicit `return`)
    FunctionDrill {
        answer: format!(
            "fn {name}({param}: i32) -> i32 {{\n    {param}\n}}"
        ),
        conditions: format!(
            "- make a function.\n\
             - function must be named \"{name}\".\n\
             - function must take one parameter named \"{param}\" of type \"i32\".\n\
             - function must return type \"i32\".\n\
             - return the parameter \"{param}\"."
        ),
    }
}

fn function_two_param_return() -> FunctionDrill {
    let names = distinct_callsigns(3);
    let name = &names[0];
    let left = &names[1];
    let right = &names[2];

    // Idiomatic: expression-style return
    FunctionDrill {
        answer: format!(
            "fn {name}({left}: i32, {right}: i32) -> i32 {{\n    {left} + {right}\n}}"
        ),
        conditions: format!(
            "- make a function.\n\
             - function must be named \"{name}\".\n\
             - function must take a parameter named \"{left}\" of type \"i32\".\n\
             - function must take a parameter named \"{right}\" of type \"i32\".\n\
             - function must return type \"i32\".\n\
             - return the sum of \"{left}\" and \"{right}\"."
        ),
    }
}

fn function_string_param() -> FunctionDrill {
    let names = distinct_callsigns(2);
    let name = &names[0];
    let param = &names[1];

    FunctionDrill {
        answer: format!(
            "fn {name}({param}: String) {{\n    println!(\"pass\");\n}}"
        ),
        conditions: format!(
            "- make a function.\n\
             - function must be named \"{name}\".\n\
             - function must take one parameter named \"{param}\" of type \"String\".\n\
             - function must not return a value.\n\
             - inside the function, print pass."
        ),
    }
}
