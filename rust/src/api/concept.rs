#[derive(Default, Debug, Copy, Clone)]
pub enum Concept {
    #[default]
    VarDecl,
    FunctionDecl,
    Ifs,
    ElseIfs,
    ForLoop,
    WhileLoop,
    Switch,
}
