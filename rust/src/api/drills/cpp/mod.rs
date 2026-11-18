use crate::api::concept::Concept;
use crate::api::drills::{
    generic_operator, generic_varname1, generic_varname2,
};

pub mod check;

#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct JavaDrill {
    pub concept: Concept,
    pub conditions: Vec<(bool, String)>,
    pub answer: String,
    pub ident_level: usize,
}

impl JavaDrill {
    #[inline]
    fn set_concept(&mut self, concept: Concept) {
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
        self.ident_level -= 1;
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
}

#[flutter_rust_bridge::frb(sync)]
pub fn java_if_statement() -> JavaDrill {
    let mut drill = JavaDrill::default();
    drill.set_concept(Concept::Ifs);
    drill.new_class("Main");
    drill.new_main();

    drill.new_if_statement(
        generic_varname1(),
        generic_operator(),
        generic_varname2(),
    );
    drill.print("pass");
    drill.new_else_statement();
    drill.print("fail");
    drill.close_if_statement();

    drill.close_main();
    drill.close_class();

    drill
}
