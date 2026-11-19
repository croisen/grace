#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Concept {
    #[default]
    VarDecl = 0,
    FunctionDecl = 1,
    Ifs = 2,
    ElseIfs = 3,
    ForLoop = 4,
    WhileLoop = 5,
    Switch = 6,
}
