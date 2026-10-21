use crate::api::drills::Comp;

#[derive(Default, Debug, Clone)]
pub struct JavaDrill {
    pub conditions: Vec<JavaCondition>,
    pub answer: String,
    ident_level: usize,
    concepts: usize,
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
        self.conditions.push(JavaCondition::ClassName(name.into()));
        self.answer.push_str(&format!("public class {name} {{\n"));

        self.ident_level += 1;
    }

    fn new_main(&mut self) {
        self.conditions.push(JavaCondition::HaveMain);
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

#[derive(Default, Debug, Clone)]
// It's more like I have made an AST here but in reverse
pub enum JavaCondition {
    ClassName(String),     // public class **Name**
    PrintOut(Vec<String>), // System.out.println(...)
    HaveMain,
    VarDecl {
        type_: String,
        name: String,
        value: Expression,
    }, // type name = value;
    Expr(Expression),
    BlockExpr(Vec<JavaCondition>),
    // Depends on how simple the conditions can be but I'm going all out on the actual AST
    IfCondition {
        condition: (String, Comp, String), // But maybe we'll just focus on a != b
        children: Box<JavaCondition>,
        else_block: Option<Box<JavaCondition>>,
    },
    ForLoop {
        init: (Option<String>, String, String), // type name = value
        condition: (String, Comp, String),
        children: Box<JavaCondition>, // depends if it's a one liner or a block expr
    },
    WhileLoop {
        condition: (String, Comp, String),
        children: Box<JavaCondition>, // depends if it's a one liner or a block expr
    },

    #[default]
    Sus,
}

#[derive(Default, Debug, Clone)]
// Yeah this is way simpler than rust that's for sure
pub enum Expression {
    Identifier(String),
    MemberAccess(Box<Expression>),
    FunctionCall {
        name: String,
        values: Vec<Expression>,
    },
    UnaryOp {
        op: String,
        operand: Box<Expression>,
    },
    BinaryOp {
        op: String,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    TernaryOp {
        condition: Box<Expression>,
        iftrue: Box<Expression>,
        iffalse: Box<Expression>,
    },

    #[default]
    Sus,
}

impl Expression {
    // idk pratt parsing yet
    pub fn new(_expr: impl AsRef<str>) -> Self {
        Self::Sus
    }
}
