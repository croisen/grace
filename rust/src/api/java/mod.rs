use rand::random_range;

use crate::api::diagnostics::Diagnostics;
use crate::api::java::check::java_check;
use crate::api::{
    generic_iterator, generic_operator, generic_varname1, generic_varname2,
};

pub mod check;

#[derive(Default, Debug, Clone, Copy)]
pub enum JavaConcept {
    #[default]
    VarDecl = 0,
    FunctionDecl = 1,
    Ifs = 2,
    ForLoop = 3,
    WhileLoop = 4,
    DoWhile = 5,
    Switch = 6,
}

impl JavaConcept {
    #[flutter_rust_bridge::frb(sync)]
    pub fn to_string(&self) -> String {
        match self {
            Self::VarDecl => "Variable Declaration".into(),
            Self::FunctionDecl => "Function Declaration".into(),
            Self::Ifs => "If Statements".into(),
            Self::ForLoop => "For Loops".into(),
            Self::WhileLoop => "While Loops".into(),
            Self::DoWhile => "Do While Loops".into(),
            Self::Switch => "Switch Case Statements".into(),
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
            "Do While Loops" => Self::DoWhile,
            "Switch Case Statements" => Self::Switch,
            _ => Self::VarDecl,
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn make_drill(&self) -> JavaDrill {
        match self {
            Self::VarDecl => java_variable_declaration(),
            Self::FunctionDecl => java_function_declaration(),
            Self::Ifs => java_if_statement(),
            Self::ForLoop => java_for_loop(),
            Self::WhileLoop => java_while_loop(),
            Self::DoWhile => java_do_while_loop(),
            JavaConcept::Switch => java_switch_statement(),
        }
    }
}

#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(opaque)]
pub struct JavaDrill {
    pub concept: JavaConcept,
    pub conditions: Vec<(bool, String)>,
    pub answer: String,
    pub ident_level: usize,
}

impl JavaDrill {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(
        concept: String,
        conditions: Vec<(bool, String)>,
        answer: String,
    ) -> Self {
        Self {
            concept: JavaConcept::from_string(concept),
            conditions,
            answer,
            ident_level: 0,
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn check(&mut self, user_code: String) -> Diagnostics {
        java_check(self.concept, &mut self.conditions, &self.answer, &user_code)
    }

    #[inline]
    fn set_concept(&mut self, concept: JavaConcept) {
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
            self.answer.push_str("  ")
        }
    }

    fn print(&mut self, thing: &str) {
        self.indent();
        self.answer.push_str(&format!("System.out.println({thing});\n"));
        self.new_condition(format!("  Print out {thing}"))
    }

    fn new_class(&mut self, name: impl AsRef<str>) {
        let name = name.as_ref();
        self.answer.push_str(&format!("public class {name} {{\n"));
        self.increase_ident();
    }

    #[inline]
    fn close_class(&mut self) {
        self.decrease_ident_with_brace(); //// }
        self.answer.push_str("\n");
    }

    fn new_main(&mut self) {
        self.indent();
        self.answer.push_str("public static void main(String[] args) {\n");
        self.increase_ident();
    }

    #[inline]
    fn close_main(&mut self) {
        self.decrease_ident_with_brace(); /////    }
        self.answer.push_str("\n");
    }

    fn new_if_statement(&mut self, var1: &str, comp: &str, var2: &str) {
        self.indent();
        self.answer.push_str(&format!("if ({var1} {comp} {var2}) {{\n"));
        self.increase_ident();
        self.new_condition(format!(
            "Make an if statement that compares {var1} and {var2} using the {comp} operator"
        ));
        self.new_condition("Inside the if statement do");
    }

    fn new_else_statement(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str(&format!(" else {{\n"));
        self.new_condition(
            "Add an else statement to the previous if statement and do",
        );
        self.increase_ident();
    }

    #[inline]
    fn close_if_statement(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str("\n");
    }

    fn new_for_loop(
        &mut self,
        var: &str,
        init: &str,
        comp: &str,
        stop: &str,
        inc_op: &str,
    ) {
        self.indent();
        self.answer.push_str(&format!(
            "for (int {var} = {init}; {var} {comp} {stop}; {var}{inc_op}) {{\n"
        ));

        self.increase_ident();

        self.new_condition(format!(
            "Create a for loop with initialization `int {var} = {init}`, condition `{var} {comp} {stop}`, and increment `{var}{inc_op}`"
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
        self.answer.push_str(&format!("int {var} = {init};\n"));
        self.new_condition(&format!(
            "Declare an integer variable named {var} and initialize it to {init}",
        ));

        self.indent();
        self.answer.push_str(&format!("while ({var} {comp} {stop}) {{\n"));

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

    fn new_switch_statement(&mut self, variable: &str) {
        self.indent();
        self.answer.push_str(&format!("switch ({variable}) {{\n"));

        self.increase_ident();

        self.new_condition(format!(
            "Create a switch statement that evaluates `{variable}`"
        ));
    }

    fn new_do_while_loop(
        &mut self,
        var: &str,
        init: &str,
        comp: &str,
        stop: &str,
    ) {
        self.indent();
        self.answer.push_str(&format!("int {var} = {init};\n"));

        self.new_condition(format!(
            "Declare an integer variable named {var} and initialize it to {init}"
        ));

        self.indent();
        self.answer.push_str("do {\n");

        self.increase_ident();

        self.new_condition("Create a do-while loop");
        self.new_condition("Inside the do-while loop do");

        self.new_condition(format!(
            "Make the loop continue while `{var} {comp} {stop}`"
        ));
    }

    fn close_do_while_loop(&mut self, var: &str, comp: &str, stop: &str) {
        self.decrease_ident_with_brace();
        self.answer.push_str(&format!(" while ({var} {comp} {stop});\n"));
    }

    fn new_case(&mut self, value: &str) {
        self.indent();
        self.answer.push_str(&format!("case {value}:\n"));
        self.increase_ident();

        self.new_condition(format!("Add a case for `{value}`"));
    }

    fn new_default_case(&mut self) {
        self.indent();
        self.answer.push_str("default:\n");
        self.increase_ident();

        self.new_condition("Add a default case");
    }

    fn close_case(&mut self) {
        self.indent();
        self.answer.push_str("break;\n");
        self.ident_level -= 1;
    }

    fn close_switch_statement(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str("\n");
    }

    fn new_function(
        &mut self,
        visibility: &str,
        return_type: &str,
        name: &str,
        parameters: &str,
    ) {
        self.indent();

        self.answer.push_str(&format!(
            "{visibility} static {return_type} {name}({parameters}) {{\n"
        ));

        self.increase_ident();

        self.new_condition(format!(
            "Declare a {visibility} static function named `{name}` \
         that returns `{return_type}` and accepts ({parameters})"
        ));

        self.new_condition("Inside the function do");
    }

    fn close_function(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str("\n");
    }
}

fn java_variable_declaration() -> JavaDrill {
    let mut drill = JavaDrill::default();
    drill.set_concept(JavaConcept::VarDecl);

    drill.new_class("Main");
    drill.new_main();

    let variables = [
        ("int", "age", "21"),
        ("double", "price", "19.99"),
        ("String", "name", "\"John\""),
    ];

    let (var_type, name, value) = variables[random_range(0..variables.len())];
    drill.indent();
    drill.answer.push_str(&format!("{var_type} {name} = {value};\n"));

    drill.new_condition(format!(
        "Declare a variable named {name} of type {var_type} and initialize it to {value}"
    ));

    drill.print(name);

    drill.close_main();
    drill.close_class();

    drill
}

fn java_if_statement() -> JavaDrill {
    let mut drill = JavaDrill::default();
    drill.set_concept(JavaConcept::Ifs);
    drill.new_class("Main");
    drill.new_main();

    drill.new_if_statement(
        generic_varname1(),
        generic_operator(),
        generic_varname2(),
    );
    drill.print("\"pass\"");
    drill.new_else_statement();
    drill.print("\"fail\"");
    drill.close_if_statement();

    drill.close_main();
    drill.close_class();

    drill
}

fn java_for_loop() -> JavaDrill {
    let mut drill = JavaDrill::default();
    drill.set_concept(JavaConcept::ForLoop);

    drill.new_class("Main");
    drill.new_main();

    let var = generic_iterator();
    let init = "0";
    let comp = generic_operator();
    let stop = &random_range(5..42).to_string();

    if comp.contains('>') {
        drill.new_for_loop(var, stop, comp, init, "--");
    } else {
        drill.new_for_loop(var, init, comp, stop, "++");
    }

    drill.print(var);

    drill.close_for_loop();

    drill.close_main();
    drill.close_class();

    drill
}

fn java_do_while_loop() -> JavaDrill {
    let mut drill = JavaDrill::default();
    drill.set_concept(JavaConcept::DoWhile);

    drill.new_class("Main");
    drill.new_main();

    let var = generic_iterator();
    let init = "0";
    let comp = generic_operator();
    let stop = random_range(5..42).to_string();

    if comp.contains('>') {
        drill.new_do_while_loop(var, &stop, comp, init);

        drill.print(var);

        drill.indent();
        drill.answer.push_str(&format!("{var}--;\n"));

        drill.new_condition(format!("Decrement {var} by 1"));
    } else {
        drill.new_do_while_loop(var, init, comp, &stop);

        drill.print(var);

        drill.indent();
        drill.answer.push_str(&format!("{var}++;\n"));

        drill.new_condition(format!("Increment {var} by 1"));
    }

    drill.close_do_while_loop(var, comp, &stop);

    drill.close_main();
    drill.close_class();

    drill
}

fn java_while_loop() -> JavaDrill {
    let mut drill = JavaDrill::default();
    drill.set_concept(JavaConcept::WhileLoop);

    drill.new_class("Main");
    drill.new_main();

    let var = generic_iterator();
    let init = "0";
    let comp = generic_operator();
    let stop = &random_range(5..42).to_string();

    if comp.contains('>') {
        drill.new_while_loop(var, stop, comp, init);
        drill.print(var);
        drill.indent();
        drill.answer.push_str(&format!("{var}--;\n"));
        drill.new_condition(&format!("Decrement {var} by 1"));
    } else {
        drill.new_while_loop(var, init, comp, stop);
        drill.print(var);
        drill.indent();
        drill.answer.push_str(&format!("{var}++;\n"));
        drill.new_condition(&format!("Increment {var} by 1"));
    }

    drill.close_while_loop();
    drill.close_main();
    drill.close_class();

    drill
}

fn java_switch_statement() -> JavaDrill {
    let mut drill = JavaDrill::default();
    drill.set_concept(JavaConcept::Switch);

    drill.new_class("Main");
    drill.new_main();

    drill.indent();
    drill.answer.push_str("int day = 2;\n");

    drill.new_condition(
        "Declare an integer variable named day and initialize it to 2",
    );

    drill.new_switch_statement("day");

    drill.new_case("1");
    drill.print("\"Monday\"");
    drill.close_case();

    drill.new_case("2");
    drill.print("\"Tuesday\"");
    drill.close_case();

    drill.new_case("3");
    drill.print("\"Wednesday\"");
    drill.close_case();

    drill.new_default_case();
    drill.print("\"Invalid day\"");
    drill.close_case();

    drill.close_switch_statement();

    drill.close_main();
    drill.close_class();

    drill
}

fn java_function_declaration() -> JavaDrill {
    let mut drill = JavaDrill::default();
    drill.set_concept(JavaConcept::FunctionDecl);

    drill.new_class("Main");

    drill.new_function("public", "int", "add", "int a, int b");

    drill.indent();
    drill.answer.push_str("return a + b;\n");

    drill.new_condition("Return the sum of a and b");

    drill.close_function();

    drill.new_main();

    drill.indent();
    drill.answer.push_str("int result = add(5, 3);\n");

    drill.new_condition(
        "Call add in main with arguments 5 and 3, and store the result in result",
    );

    drill.print("result");

    drill.close_main();
    drill.close_class();

    drill
}
