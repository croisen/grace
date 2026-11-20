use rand::random_range;

use crate::api::{
    generic_iterator, generic_operator, generic_varname1, generic_varname2,
};

pub mod check;

#[derive(Default, Debug, Clone)]
pub enum JavaConcept {
    #[default]
    VarDecl = 0,
    FunctionDecl = 1,
    Ifs = 2,
    ElseIfs = 3,
    ForLoop = 4,
    WhileLoop = 5,
    DoWhile = 6,
    Switch = 7,
}

impl JavaConcept {
    #[flutter_rust_bridge::frb(non_opaque)]
    pub fn to_string(&self) -> String {
        match self {
            Self::VarDecl => "Variable Declaration".into(),
            Self::FunctionDecl => "Function Declaration".into(),
            Self::Ifs => "If Statements".into(),
            Self::ElseIfs => "If Else If Statements".into(),
            Self::ForLoop => "For Loops".into(),
            Self::WhileLoop => "While Loops".into(),
            Self::DoWhile => "Do While Loops".into(),
            Self::Switch => "Switch Case Statements".into(),
        }
    }

    pub fn from_string(s: String) -> Self {
        match s.as_str() {
            "Variable Declaration" => Self::VarDecl,
            "Function Declaration" => Self::FunctionDecl,
            "If Statements" => Self::Ifs,
            "If Else If Statements" => Self::ElseIfs,
            "For Loops" => Self::ForLoop,
            "While Loops" => Self::WhileLoop,
            "Do While Loops" => Self::DoWhile,
            "Switch Case Statements" => Self::Switch,
            _ => Self::VarDecl,
        }
    }
}

#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct JavaDrill {
    pub concept: JavaConcept,
    pub conditions: Vec<(bool, String)>,
    pub answer: String,
    pub ident_level: usize,
}

impl JavaDrill {
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
        self.decrease_ident_with_brace();
        self.answer.push_str("\n");
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

#[flutter_rust_bridge::frb(sync)]
pub fn java_if_statement() -> JavaDrill {
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

#[flutter_rust_bridge::frb(sync)]
pub fn java_for_loop() -> JavaDrill {
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

#[flutter_rust_bridge::frb(sync)]
pub fn java_while_loop() -> JavaDrill {
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
        drill.new_condition(&format!("Decrement {var} by 1"));
    }

    drill.close_while_loop();
    drill.close_main();
    drill.close_class();

    drill
}

#[flutter_rust_bridge::frb(sync)]
pub fn java_switch_statement() -> JavaDrill {
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

#[flutter_rust_bridge::frb(sync)]
pub fn java_function_declaration() -> JavaDrill {
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
        "Call add with arguments 5 and 3, and store the result in result",
    );

    drill.print("result");

    drill.close_main();
    drill.close_class();

    drill
}
