#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct JavaDrill {
    pub conditions: Vec<String>,
    pub answer: String,
    pub ident_level: usize,
}

impl JavaDrill {
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
        self.conditions.push(format!("  Print out {thing}"))
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
        // self.conditions.push(JavaCondition::HaveMain);
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
        self.conditions.push(format!(
            "Make an if statement that compares {var1} and {var2}"
        ));
        self.conditions.push(format!("Use the {comp} operator"));
        self.conditions.push("Inside the if statement do".into());
    }

    fn new_else_statement(&mut self) {
        self.decrease_ident_with_brace();
        self.answer.push_str(&format!(" else {{\n"));
        self.conditions.push(
            "Add an else statement to the previous if statement and do".into(),
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
    drill.new_class("Main");
    drill.new_main();

    drill.new_if_statement("a", "<=", "b");
    drill.print("pass");
    drill.print("yadada");
    drill.close_if_statement();

    drill.close_main();
    drill.close_class();

    println!();
    println!("{}", drill.answer);
    println!();
    for cond in &drill.conditions {
        println!("- {cond}");
    }

    drill
}
