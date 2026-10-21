pub mod java;
pub mod rust;

#[derive(Default, Debug, Clone)]
pub enum Comp {
    GE,  // >=
    GT,  // >
    LE,  // <=
    LT,  // <
    EQ,  // ==
    NE,  // !=
    AND, // &&
    OR,  // ||
    NEG, // !

    #[default]
    SUS,
}
