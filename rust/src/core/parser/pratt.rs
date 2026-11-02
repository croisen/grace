#[flutter_rust_bridge::frb(ignore)]
pub enum PrattAssociativity {
    Left,
    Right,
    None,
}

#[flutter_rust_bridge::frb(ignore)]
pub struct PrattOperator {
    pub precedence: usize,
    pub associativity: PrattAssociativity,
}

impl PrattOperator {
    pub fn left(precedence: usize) -> Self {
        Self { precedence, associativity: PrattAssociativity::Left }
    }

    pub fn right(precedence: usize) -> Self {
        Self { precedence, associativity: PrattAssociativity::Right }
    }

    pub fn none(precedence: usize) -> Self {
        Self { precedence, associativity: PrattAssociativity::None }
    }
}

#[flutter_rust_bridge::frb(ignore)]
pub trait ParserPratt<'a, TK, LangExpression>
where
    TK: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>
        + Default
        + Debug,
{
}
