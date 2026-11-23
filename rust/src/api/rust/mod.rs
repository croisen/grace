use rand::{random, random_range};

use crate::api::diagnostics::Diagnostics;
use crate::api::rust::check::rust_check;
use crate::api::{
    generic_iterator, generic_operator, generic_varname1, generic_varname2,
};

pub mod check;

#[derive(Default, Debug, Clone, Copy)]
pub enum RustConcept {
    #[default]
    VarDecl = 0,
    FunctionDecl = 1,
    Ifs = 2,
    ForLoop = 3,
    WhileLoop = 4,
    Loop = 5,
    Match = 6,
}

impl RustConcept {
    #[flutter_rust_bridge::frb(sync)]
    pub fn to_string(&self) -> String {
        match self {
            Self::VarDecl => "Variable Declaration".into(),
            Self::FunctionDecl => "Function Declaration".into(),
            Self::Ifs => "If Statements".into(),
            Self::ForLoop => "For Loops".into(),
            Self::WhileLoop => "While Loops".into(),
            Self::Loop => "Loop Statements".into(),
            Self::Match => "Match Statements".into(),
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn from_string(s: String) -> Self {
        match s.as_str() {
            "Variable Declaration" => Self::VarDecl,
            "Function Declaration" => Self::FunctionDecl,
            "If Statements" => Self::Ifs,
            "For Loops" => Self::ForLoop,
            "While Loops" => Self::WhileLoop,
            "Loop Statements" => Self::Loop,
            "Match Statements" => Self::Match,
            _ => Self::VarDecl,
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn make_drill(&self) -> RustDrill {
        match self {
            Self::VarDecl => rust_variable_declaration(),
            Self::FunctionDecl => rust_function_declaration(),
            Self::Ifs => rust_if_statement(),
            Self::ForLoop => rust_for_loop(),
            Self::WhileLoop => rust_while_loop(),
            Self::Loop => rust_loop(),
            Self::Match => rust_match_statement(),
        }
    }
}

#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(opaque)]
pub struct RustDrill {
    pub concept: RustConcept,
    pub conditions: Vec<(bool, String)>,
    pub answer: String,
    pub ident_level: usize,
}

impl RustDrill {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(
        concept: String,
        conditions: Vec<(bool, String)>,
        answer: String,
    ) -> Self {
        Self {
            concept: RustConcept::from_string(concept),
            conditions,
            answer,
            ident_level: 0,
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn check(&mut self, user_code: String) -> Diagnostics {
        rust_check(self.concept, &mut self.conditions, &self.answer, &user_code)
    }

    #[inline]
    fn set_concept(&mut self, concept: RustConcept) {
        self.concept = concept;
    }

    #[inline]
    fn new_condition(&mut self, condition: impl Into<String>) {
        self.conditions.push((false, condition.into()));
    }

    #[inline]
    fn increase_ident(&mut self) {
        self.ident_level += 1;
    }

    #[inline]
    fn decrease_ident_with_brace(&mut self) {
        self.ident_level = self.ident_level.saturating_sub(1);
        self.indent();
        self.answer.push_str("}");
    }

    #[inline]
    fn indent(&mut self) {
        for _ in 0..self.ident_level {
            self.answer.push_str("  ");
        }
    }

    fn print(&mut self, thing: &str) {
        self.indent();
        self.answer.push_str(&format!("println!(\"{{}}\", {thing});\n"));
        self.new_condition(format!("  Print out {thing}"));
    }

    fn new_main(&mut self) {
        self.answer.push_str("fn main() {\n");
        self.increase_ident();
    }

    #[inline]
    fn close_main(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str("\n");
    }

    fn new_if_statement(&mut self, var1: &str, comp: &str, var2: &str) {
        self.indent();

        self.answer.push_str(&format!("if {var1} {comp} {var2} {{\n"));

        self.increase_ident();

        self.new_condition(format!(
            "Make an if statement that compares {var1} and {var2} using the {comp} operator"
        ));

        self.new_condition("Inside the if statement do");
    }

    fn new_else_statement(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str(" else {\n");

        self.new_condition(
            "Add an else statement to the previous if statement and do",
        );

        self.increase_ident();
    }

    fn close_if_statement(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str("\n");
    }

    fn new_else_if_statement(&mut self, var1: &str, comp: &str, var2: &str) {
        self.decrease_ident_with_brace();

        self.answer.push_str(&format!(" else if {var1} {comp} {var2} {{\n"));

        self.increase_ident();

        self.new_condition(format!(
            "Add an else if statement comparing {var1} and {var2} using {comp}"
        ));

        self.new_condition("Inside the else if statement do");
    }

    fn new_for_loop(&mut self, var: &str, start: &str, stop: &str) {
        self.indent();

        self.answer.push_str(&format!("for {var} in {start}..{stop} {{\n"));

        self.increase_ident();

        self.new_condition(format!(
            "Create a for loop using `{var} in {start}..{stop}`"
        ));

        self.new_condition("Inside the for loop do");
    }

    fn close_for_loop(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str("\n");
    }

    fn new_while_loop(
        &mut self,
        var: &str,
        init: &str,
        comp: &str,
        stop: &str,
    ) {
        self.indent();

        self.answer.push_str(&format!("let mut {var} = {init};\n"));

        self.new_condition(format!(
            "Declare a mutable integer variable named {var} and initialize it to {init}"
        ));

        self.indent();

        self.answer.push_str(&format!("while {var} {comp} {stop} {{\n"));

        self.increase_ident();

        self.new_condition(format!(
            "Create a while loop that continues while `{var} {comp} {stop}`"
        ));

        self.new_condition("Inside the while loop do");
    }

    fn close_while_loop(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str("\n");
    }

    fn new_loop(&mut self) {
        self.indent();
        self.answer.push_str("loop {\n");

        self.increase_ident();

        self.new_condition("Create an infinite loop using the loop keyword");

        self.new_condition("Inside the loop do");
    }

    fn close_loop(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str("\n");
    }

    fn new_match_statement(&mut self, variable: &str) {
        self.indent();

        self.answer.push_str(&format!("match {variable} {{\n"));

        self.increase_ident();

        self.new_condition(format!(
            "Create a match statement that evaluates `{variable}`"
        ));
    }

    fn new_match_arm(&mut self, value: &str) {
        self.indent();

        self.answer.push_str(&format!("{value} => {{\n"));

        self.increase_ident();

        self.new_condition(format!("Add a match arm for `{value}`"));
    }

    fn new_default_match_arm(&mut self) {
        self.indent();

        self.answer.push_str("_ => {\n");

        self.increase_ident();

        self.new_condition("Add a default match arm using `_`");
    }

    fn close_match_arm(&mut self) {
        self.decrease_ident_with_brace();

        self.answer.push_str(",\n");
    }

    fn close_match_statement(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str("\n");
    }

    fn new_function(
        &mut self,
        visibility: &str,
        name: &str,
        parameters: &str,
        return_type: &str,
    ) {
        self.indent();

        let visibility = if visibility.is_empty() {
            String::new()
        } else {
            format!("{visibility} ")
        };

        let return_type = if return_type.is_empty() {
            String::new()
        } else {
            format!(" -> {return_type}")
        };

        self.answer.push_str(&format!(
            "{visibility}fn {name}({parameters}){return_type} {{\n"
        ));

        self.increase_ident();

        self.new_condition(format!(
            "Declare a {visibility}function named `{name}` that accepts ({parameters}) and returns `{return_type}`"
        ));

        self.new_condition("Inside the function do");
    }

    fn close_function(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str("\n");
    }
}

fn rust_variable_declaration() -> RustDrill {
    let mut drill = RustDrill::default();

    drill.set_concept(RustConcept::VarDecl);
    drill.new_main();

    let exercise = random_range(0..2);

    match exercise {
        0 => {
            // Immutable variable with inferred type
            let var = generic_varname1();
            let val = random_range(19..63);

            drill.indent();
            drill.answer.push_str(&format!("let {var} = {val};\n"));

            drill.new_condition(format!(
                "Declare an immutable variable named {var} and initialize it to {val}",
            ));

            drill.print(var);
        }
        1 => {
            // Mutable variable
            drill.indent();
            let var = generic_varname2();
            let val = random_range(63..1412);
            drill.answer.push_str(&format!("let mut {var} = {val};\n"));

            drill.new_condition(format!(
                "Declare a mutable integer variable named {var} and initialize it to {val}"
            ));

            drill.print(var);

            // Reassign
            let val = random_range(63..1412);
            drill.indent();
            drill.answer.push_str(&format!("{var} = {val};\n"));

            drill.new_condition(format!("Reassign {var} to {val}"));

            drill.print(var);
        }
        2 => {
            let var = generic_varname2();
            let val = random::<f64>();
            drill.indent();
            drill.answer.push_str(&format!("let {var}: f64 = {val:>4.2};\n"));

            drill.new_condition(format!(
                "Declare an immutable variable named {var} with an explicit f64 type and initialize it to {val:>4.2}"
            ));

            drill.print(var);
        }
        _ => unreachable!(),
    }

    drill.close_main();

    drill
}

fn rust_if_statement() -> RustDrill {
    let mut drill = RustDrill::default();

    drill.set_concept(RustConcept::Ifs);

    drill.new_main();

    drill.new_if_statement(
        &generic_varname1(),
        &generic_operator(),
        &generic_varname2(),
    );

    drill.print("\"pass\"");

    drill.new_else_statement();

    drill.print("\"fail\"");

    drill.close_if_statement();

    drill.close_main();

    drill
}

fn rust_for_loop() -> RustDrill {
    let mut drill = RustDrill::default();

    drill.set_concept(RustConcept::ForLoop);

    drill.new_main();

    let var = generic_iterator();

    let start = 0;
    let stop = random_range(5..42);

    drill.new_for_loop(var, &start.to_string(), &stop.to_string());

    drill.print(var);

    drill.close_for_loop();

    drill.close_main();

    drill
}

fn rust_while_loop() -> RustDrill {
    let mut drill = RustDrill::default();

    drill.set_concept(RustConcept::WhileLoop);

    drill.new_main();

    let var = generic_iterator();
    let stop = random_range(5..42);

    drill.new_while_loop(var, "0", "<", &stop.to_string());

    drill.print(var);

    drill.indent();
    drill.answer.push_str(&format!("{var} += 1;\n"));

    drill.new_condition(format!("Increment {var} by 1"));

    drill.close_while_loop();

    drill.close_main();

    drill
}

fn rust_loop() -> RustDrill {
    let mut drill = RustDrill::default();

    drill.set_concept(RustConcept::Loop);

    drill.new_main();

    drill.new_loop();

    drill.print("\"Hello, Rust!\"");

    drill.indent();
    drill.answer.push_str("break;\n");

    drill.new_condition("Exit the loop using break");

    drill.close_loop();

    drill.close_main();

    drill
}

fn rust_match_statement() -> RustDrill {
    let mut drill = RustDrill::default();

    drill.set_concept(RustConcept::Match);

    drill.new_main();

    drill.indent();
    drill.answer.push_str("let day = 2;\n");

    drill.new_condition(
        "Declare an immutable integer variable named day and initialize it to 2",
    );

    drill.new_match_statement("day");

    drill.new_match_arm("1");

    drill.print("\"Monday\"");

    drill.close_match_arm();

    drill.new_match_arm("2");

    drill.print("\"Tuesday\"");

    drill.close_match_arm();

    drill.new_match_arm("3");

    drill.print("\"Wednesday\"");

    drill.close_match_arm();

    drill.new_default_match_arm();

    drill.print("\"Invalid day\"");

    drill.close_match_arm();

    drill.close_match_statement();

    drill.close_main();

    drill
}

fn rust_function_declaration() -> RustDrill {
    let mut drill = RustDrill::default();

    drill.set_concept(RustConcept::FunctionDecl);

    drill.new_function("", "add", "a: i32, b: i32", "i32");

    drill.indent();
    drill.answer.push_str("a + b\n");

    drill.new_condition("Return the sum of a and b");

    drill.close_function();

    drill.new_main();

    drill.indent();
    drill.answer.push_str("let result = add(5, 3);\n");

    drill.new_condition(
        "Call add with arguments 5 and 3, and store the result in result",
    );

    drill.print("result");

    drill.close_main();

    drill
}
