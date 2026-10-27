pub mod else_if;
pub mod for_statement;
pub mod function;
pub mod if_statement;
pub mod loop_statement;
pub mod match_statement;
pub mod variable;
pub mod while_statement;

use rand::RngExt;

fn callsign() -> String {
    let mut rng = rand::rng();

    let names = [
        "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf",
        "hotel", "india", "juliett", "kilo", "lima", "mike", "november",
        "oscar", "papa", "quebec", "romeo", "sierra", "tango", "uniform",
        "victor", "whiskey", "xray", "yankee", "zulu",
    ];

    names[rng.random_range(0..names.len())].to_string()
}

fn distinct_callsigns(count: usize) -> Vec<String> {
    assert!(count <= 26, "only 26 callsigns exist");

    let mut names = Vec::with_capacity(count);

    while names.len() < count {
        let name = callsign();

        if names.contains(&name) {
            continue;
        }

        names.push(name);
    }

    names
}

#[cfg(test)]
mod tests {
    use super::else_if::generate_else_if;
    use super::for_statement::generate_for_statement;
    use super::function::generate_function;
    use super::if_statement::generate_if_statement;
    use super::loop_statement::generate_loop_statement;
    use super::match_statement::generate_match_statement;
    use super::variable::generate_variable;
    use super::while_statement::generate_while_statement;

    #[test]
    fn generators_produce_syntax_and_conditions() {
        for _ in 0..40 {
            let variable = generate_variable(1, 0);
            assert!(variable.code.starts_with("let "));
            assert!(variable.conditions.contains(&variable.name));

            let if_drill = generate_if_statement();
            assert!(if_drill.answer.contains("if "));
            assert!(if_drill.answer.contains("else"));
            assert!(if_drill.conditions.contains("if statement"));

            let else_if = generate_else_if();
            assert!(else_if.answer.contains("else if "));
            assert!(else_if.conditions.contains("else if"));

            let match_drill = generate_match_statement();
            assert!(match_drill.answer.contains("match "));
            assert!(match_drill.answer.contains("_ =>"));
            assert!(match_drill.conditions.contains("match"));

            let while_drill = generate_while_statement();
            assert!(while_drill.answer.contains("while "));
            assert!(while_drill.conditions.contains("while"));

            let for_drill = generate_for_statement();
            assert!(for_drill.answer.contains("for "));
            assert!(for_drill.answer.contains(".."));
            assert!(for_drill.conditions.contains("for loop"));

            let loop_drill = generate_loop_statement();
            assert!(loop_drill.answer.contains("loop {"));
            assert!(loop_drill.answer.contains("break;"));
            assert!(loop_drill.conditions.contains("loop"));

            let function = generate_function();
            assert!(function.answer.starts_with("fn "));
            assert!(function.conditions.contains("function"));
        }
    }
}
