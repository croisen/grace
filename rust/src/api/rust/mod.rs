use rand::random_range;

pub mod check;

// -----------------------------------------------------------------------------
// Public drill types
// -----------------------------------------------------------------------------
// Keep these names and fields stable: the existing Flutter Rust Bridge API
// exposes one return type for each drill, and the Dart side reads `answer` and
// `conditions` directly.

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct VariableDrill {
    pub code: String,
    pub conditions: String,
    pub name: String,
    pub state: String,
    pub var_type: String,
    pub value: String,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct IfStatementDrill {
    pub answer: String,
    pub conditions: String,
}

#[derive(Debug, Clone)]
pub struct ElseIfDrill {
    pub answer: String,
    pub conditions: String,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct MatchStatementDrill {
    pub answer: String,
    pub conditions: String,
}

#[derive(Debug, Clone)]
pub struct WhileStatementDrill {
    pub answer: String,
    pub conditions: String,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct ForStatementDrill {
    pub answer: String,
    pub conditions: String,
}

#[derive(Debug, Clone)]
pub struct LoopStatementDrill {
    pub answer: String,
    pub conditions: String,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct FunctionDrill {
    pub answer: String,
    pub conditions: String,
}

// -----------------------------------------------------------------------------
// Shared Rust drill builder
// -----------------------------------------------------------------------------

#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
struct RustDrill {
    answer: String,
    conditions: Vec<String>,
    ident_level: usize,
}

impl RustDrill {
    #[inline]
    fn new_condition(&mut self, condition: impl Into<String>) {
        self.conditions.push(condition.into());
    }

    #[inline]
    fn increase_ident(&mut self) {
        self.ident_level += 1;
    }

    #[inline]
    fn decrease_ident(&mut self) {
        self.ident_level = self.ident_level.saturating_sub(1);
    }

    #[inline]
    fn indent(&mut self) {
        for _ in 0..self.ident_level {
            self.answer.push_str("    ");
        }
    }

    fn line(&mut self, source: impl AsRef<str>) {
        self.indent();
        self.answer.push_str(source.as_ref());
        self.answer.push('\n');
    }

    #[inline]
    fn blank_line(&mut self) {
        self.answer.push('\n');
    }

    fn open_block(&mut self, header: &str) {
        self.line(format!("{header} {{"));
        self.increase_ident();
    }

    #[inline]
    fn close_block(&mut self) {
        self.decrease_ident();
        self.indent();
        self.answer.push_str("}\n");
    }

    fn close_block_with_clause(&mut self, clause: &str) {
        self.decrease_ident();
        self.indent();
        self.answer.push_str("} ");
        self.answer.push_str(clause);
        self.answer.push_str(" {\n");
        self.increase_ident();
    }

    fn new_variable(&mut self, variable: &VariableDrill) {
        self.line(&variable.code);
        self.new_condition(variable.conditions.clone());
    }

    fn print(&mut self, value: &str) {
        self.line(format!("println!({value});"));
        self.new_condition(format!("Print out {value}"));
    }

    fn new_if_statement(&mut self, left: &str, operator: &str, right: &str) {
        self.open_block(&format!("if {left} {operator} {right}"));
        self.new_condition(format!(
            "Make an if statement that compares {left} and {right} using the {operator} operator"
        ));
        self.new_condition("Inside the if statement do");
    }

    fn new_else_if_statement(
        &mut self,
        left: &str,
        operator: &str,
        right: &str,
    ) {
        self.close_block_with_clause(&format!(
            "else if {left} {operator} {right}"
        ));
        self.new_condition(format!(
            "Make an else if statement that compares {left} and {right} using the {operator} operator"
        ));
        self.new_condition("Inside the else if statement do");
    }

    fn new_else_statement(&mut self) {
        self.close_block_with_clause("else");
        self.new_condition(
            "Add an else statement to the previous if statement and do",
        );
    }

    fn close_if_statement(&mut self) {
        self.close_block();
    }

    fn new_for_loop(&mut self, variable: &str, start: i32, end: i32) {
        self.open_block(&format!("for {variable} in {start}..{end}"));
        self.new_condition(format!(
            "Create a for loop using `{variable}` that iterates from {start} up to but not including {end}"
        ));
        self.new_condition("Inside the for loop do");
    }

    fn close_for_loop(&mut self) {
        self.close_block();
    }

    fn new_while_loop(&mut self, variable: &str, bound: i32) {
        self.open_block(&format!("while {variable} < {bound}"));
        self.new_condition(format!(
            "Create a while loop that continues while `{variable} < {bound}`"
        ));
        self.new_condition("Inside the while loop do");
    }

    fn close_while_loop(&mut self) {
        self.close_block();
    }

    fn new_loop_statement(&mut self) {
        self.open_block("loop");
        self.new_condition("Create an infinite loop using `loop`");
    }

    fn new_match_statement(&mut self, variable: &str) {
        self.open_block(&format!("match {variable}"));
        self.new_condition(format!(
            "Create a match statement that matches on `{variable}`"
        ));
    }

    fn new_match_case(&mut self, value: i32, output: &str) {
        self.line(format!("{value} => println!({output}),"));
        self.new_condition(format!(
            "Add a match arm for `{value}` that prints {output}"
        ));
    }

    fn new_default_match_case(&mut self, output: &str) {
        self.line(format!("_ => println!({output}),"));
        self.new_condition(format!(
            "Add a default match arm that prints {output} for any other value"
        ));
    }

    fn new_function(
        &mut self,
        name: &str,
        parameters: &str,
        return_type: Option<&str>,
        instructions: String,
    ) {
        let return_type =
            return_type.map(|ty| format!(" -> {ty}")).unwrap_or_default();

        self.open_block(&format!("fn {name}({parameters}){return_type}"));
        self.new_condition(instructions);
        self.new_condition("Inside the function do");
    }

    fn finish(self) -> (String, String) {
        let answer = self.answer.trim_end_matches('\n').to_string();
        (answer, self.conditions.join("\n\n"))
    }
}

// -----------------------------------------------------------------------------
// Shared name and variable helpers
// -----------------------------------------------------------------------------

fn callsign() -> String {
    const NAMES: &[&str] = &[
        "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf",
        "hotel", "india", "juliett", "kilo", "lima", "mike", "november",
        "oscar", "papa", "quebec", "romeo", "sierra", "tango", "uniform",
        "victor", "whiskey", "xray", "yankee", "zulu",
    ];

    NAMES[random_range(0..NAMES.len())].to_string()
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

pub fn generate_variable(choice: usize, if_mut: usize) -> VariableDrill {
    let var_type = match choice {
        0 => "String",
        1 => "i32",
        _ => "String",
    };

    let name = callsign();
    let value = match choice {
        0 => callsign(),
        1 => random_range(0..=257).to_string(),
        _ => callsign(),
    };

    let (is_mut, state) = match if_mut {
        1 => ("must be mutable.", " mut"),
        0 => ("must not be mutable.", ""),
        _ => ("may or may not be mutable.", ""),
    };

    let var_value = if var_type == "String" {
        format!("String::from({value:?})")
    } else {
        value.clone()
    };

    VariableDrill {
        code: format!("let{state} {name}: {var_type} = {var_value};"),
        conditions: format!(
            "- variable must be named \"{name}\"\n\
             - variable \"{name}\" must be of type \"{var_type}\"\n\
             - variable \"{name}\" must have a value of \"{value}\"\n\
             - variable \"{name}\" {is_mut}"
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

    let mut variables = Vec::with_capacity(count);

    while variables.len() < count {
        let next = generate_variable(choice, if_mut);

        if variables
            .iter()
            .any(|variable: &VariableDrill| variable.name == next.name)
        {
            continue;
        }

        variables.push(next);
    }

    variables
}

// -----------------------------------------------------------------------------
// Drill generators
// -----------------------------------------------------------------------------

#[flutter_rust_bridge::frb(sync)]
pub fn generate_if_statement() -> IfStatementDrill {
    let variables = generate_distinct_variables(2, 1, 0);
    let operator = ["==", "<=", ">=", "<", ">"][random_range(0..5)];

    let mut drill = RustDrill::default();
    drill.new_variable(&variables[0]);
    drill.new_variable(&variables[1]);
    drill.blank_line();

    drill.new_if_statement(&variables[0].name, operator, &variables[1].name);
    drill.print("\"pass\"");
    drill.new_else_statement();
    drill.print("\"fail\"");
    drill.close_if_statement();

    let (answer, conditions) = drill.finish();
    IfStatementDrill { answer, conditions }
}

#[flutter_rust_bridge::frb(sync)]
pub fn generate_else_if() -> ElseIfDrill {
    let variables = generate_distinct_variables(3, 1, 0);
    let operators = ["==", "<=", ">=", "<", ">"];
    let first_operator = operators[random_range(0..operators.len())];
    let second_operator = operators[random_range(0..operators.len())];

    let mut drill = RustDrill::default();
    for variable in &variables {
        drill.new_variable(variable);
    }
    drill.blank_line();

    drill.new_if_statement(
        &variables[0].name,
        first_operator,
        &variables[1].name,
    );
    drill.print("\"pass\"");
    drill.new_else_if_statement(
        &variables[0].name,
        second_operator,
        &variables[2].name,
    );
    drill.print("\"retry\"");
    drill.new_else_statement();
    drill.print("\"fail\"");
    drill.close_if_statement();

    let (answer, conditions) = drill.finish();
    ElseIfDrill { answer, conditions }
}

#[flutter_rust_bridge::frb(sync)]
pub fn generate_match_statement() -> MatchStatementDrill {
    let mut literals = Vec::with_capacity(2);

    while literals.len() < 2 {
        let literal = random_range(0..=9);
        if !literals.contains(&literal) {
            literals.push(literal);
        }
    }
    literals.sort_unstable();

    let variable = generate_variable(1, 0);
    let mut drill = RustDrill::default();
    drill.new_variable(&variable);
    drill.blank_line();
    drill.new_match_statement(&variable.name);
    drill.new_match_case(literals[0], "\"pass\"");
    drill.new_match_case(literals[1], "\"retry\"");
    drill.new_default_match_case("\"fail\"");
    drill.close_block();

    let (answer, conditions) = drill.finish();
    MatchStatementDrill { answer, conditions }
}

#[flutter_rust_bridge::frb(sync)]
pub fn generate_while_statement() -> WhileStatementDrill {
    let bound = random_range(3..=8);
    let variable = generate_variable(1, 1);

    let mut drill = RustDrill::default();
    drill.new_variable(&variable);
    drill.blank_line();
    drill.new_while_loop(&variable.name, bound);
    drill.print("\"pass\"");
    drill.line(format!("{} += 1;", variable.name));
    drill.new_condition(format!("Increment {} by 1", variable.name));
    drill.close_while_loop();

    let (answer, conditions) = drill.finish();
    WhileStatementDrill { answer, conditions }
}

#[flutter_rust_bridge::frb(sync)]
pub fn generate_for_statement() -> ForStatementDrill {
    let variable = distinct_callsigns(1).remove(0);
    let start = random_range(0..=5);
    let end = random_range((start + 1)..=(start + 10));

    let mut drill = RustDrill::default();
    drill.new_for_loop(&variable, start, end);
    drill.print("\"pass\"");
    drill.close_for_loop();

    let (answer, conditions) = drill.finish();
    ForStatementDrill { answer, conditions }
}

#[flutter_rust_bridge::frb(sync)]
pub fn generate_loop_statement() -> LoopStatementDrill {
    let bound = random_range(3..=8);
    let variable = generate_variable(1, 1);

    let mut drill = RustDrill::default();
    drill.new_variable(&variable);
    drill.blank_line();
    drill.new_loop_statement();

    drill.new_if_statement(&variable.name, ">=", &bound.to_string());
    drill.line("break;");
    drill.close_if_statement();

    drill.print("\"pass\"");
    drill.line(format!("{} += 1;", variable.name));
    drill.new_condition(format!("Increment {} by 1", variable.name));
    drill.close_block();

    let (answer, conditions) = drill.finish();
    LoopStatementDrill { answer, conditions }
}

#[flutter_rust_bridge::frb(sync)]
pub fn generate_function() -> FunctionDrill {
    match random_range(0..4) {
        0 => function_no_params(),
        1 => function_one_param_return(),
        2 => function_two_param_return(),
        _ => function_string_param(),
    }
}

fn function_no_params() -> FunctionDrill {
    let name = distinct_callsigns(1).remove(0);
    let mut drill = RustDrill::default();

    drill.new_function(
        &name,
        "",
        None,
        format!(
            "- make a function.\n\
             - function must be named \"{name}\".\n\
             - function must not take any parameters.\n\
             - function must not return a value.\n\
             - inside the function, print pass."
        ),
    );
    drill.print("\"pass\"");
    drill.close_block();

    let (answer, conditions) = drill.finish();
    FunctionDrill { answer, conditions }
}

fn function_one_param_return() -> FunctionDrill {
    let names = distinct_callsigns(2);
    let name = &names[0];
    let parameter = &names[1];
    let mut drill = RustDrill::default();

    drill.new_function(
        name,
        &format!("{parameter}: i32"),
        Some("i32"),
        format!(
            "- make a function.\n\
             - function must be named \"{name}\".\n\
             - function must take one parameter named \"{parameter}\" of type \"i32\".\n\
             - function must return type \"i32\".\n\
             - return the parameter \"{parameter}\"."
        ),
    );
    drill.line(parameter);
    drill.close_block();

    let (answer, conditions) = drill.finish();
    FunctionDrill { answer, conditions }
}

fn function_two_param_return() -> FunctionDrill {
    let names = distinct_callsigns(3);
    let name = &names[0];
    let left = &names[1];
    let right = &names[2];
    let mut drill = RustDrill::default();

    drill.new_function(
        name,
        &format!("{left}: i32, {right}: i32"),
        Some("i32"),
        format!(
            "- make a function.\n\
             - function must be named \"{name}\".\n\
             - function must take a parameter named \"{left}\" of type \"i32\".\n\
             - function must take a parameter named \"{right}\" of type \"i32\".\n\
             - function must return type \"i32\".\n\
             - return the sum of \"{left}\" and \"{right}\"."
        ),
    );
    drill.line(format!("{left} + {right}"));
    drill.close_block();

    let (answer, conditions) = drill.finish();
    FunctionDrill { answer, conditions }
}

fn function_string_param() -> FunctionDrill {
    let names = distinct_callsigns(2);
    let name = &names[0];
    let parameter = &names[1];
    let mut drill = RustDrill::default();

    drill.new_function(
        name,
        &format!("{parameter}: String"),
        None,
        format!(
            "- make a function.\n\
             - function must be named \"{name}\".\n\
             - function must take one parameter named \"{parameter}\" of type \"String\".\n\
             - function must not return a value.\n\
             - inside the function, print pass."
        ),
    );
    drill.print("\"pass\"");
    drill.close_block();

    let (answer, conditions) = drill.finish();
    FunctionDrill { answer, conditions }
}

#[cfg(test)]
mod tests {
    use super::{
        generate_distinct_variables, generate_else_if, generate_for_statement,
        generate_function, generate_if_statement, generate_loop_statement,
        generate_match_statement, generate_variable, generate_while_statement,
    };

    #[test]
    fn generators_produce_rust_snippets_and_prompts() {
        for _ in 0..40 {
            let variable = generate_variable(1, 0);
            assert!(variable.code.starts_with("let "));
            assert!(variable.conditions.contains(&variable.name));

            let distinct = generate_distinct_variables(3, 1, 0);
            assert_ne!(distinct[0].name, distinct[1].name);
            assert_ne!(distinct[0].name, distinct[2].name);
            assert_ne!(distinct[1].name, distinct[2].name);

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
            assert!(while_drill.answer.contains("+= 1;"));
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
