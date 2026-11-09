#[derive(Default, Debug, Clone)]
pub struct JavaDrill {
    pub conditions: Vec<String>,
    pub answer: String,
    pub ident_level: usize,
    pub concepts: usize,
}

impl JavaDrill {
    // how many conditions are there supposed to be, except for class
    // but maybe we'll see if we ditch the class name itself afterwards
    pub fn new(concepts: usize) -> Self {
        let mut drill = Self::default();
        drill.concepts = concepts;

        drill.new_class("Main"); ////////////// public class Main {
        drill.new_main(); //////////////////////    public static void main(String[] args) {
        drill.decrease_ident_with_brace(); /////    }
        drill.decrease_ident_with_brace(); //// }
        drill
    }

    fn new_class(&mut self, name: impl AsRef<str>) {
        let name = name.as_ref();
        // self.conditions.push(JavaCondition::ClassName(name.into()));
        self.answer.push_str(&format!("public class {name} {{\n"));

        self.ident_level += 1;
    }

    fn new_main(&mut self) {
        // self.conditions.push(JavaCondition::HaveMain);
        self.indent();
        self.answer.push_str("public static void main(String[] args) {{\n");

        self.ident_level += 1;
    }

    fn decrease_ident_with_brace(&mut self) {
        self.ident_level -= 1;
        self.indent();
        self.answer.push_str("}");
    }

    fn indent(&mut self) {
        for _ in 0..self.ident_level {
            self.answer.push_str("  ")
        }
    }
}
