use crate::core::parser::pratt::Token2;
use crate::langs::rust::ast::RustParserState;

#[derive(Debug, Clone)]
pub enum RustVisibility {
    Pub,
    PubCrate,
    PubSelf,
    PubSuper,
    PubIn(Vec<Token2>),
}

impl RustVisibility {
    pub fn new(state: &mut RustParserState) -> Option<Self> {
        if !state.token_take_text_sen("pub") {
            return None;
        }
        if !state.token_eat_text_sen("(") {
            return Some(Self::Pub);
        }
        state.token_take_text_sen("(");
        let ret = if state.token_take_text_sen("crate") {
            state.token_expect_take_text_sen(")");

            Self::PubCrate
        } else if state.token_take_text_sen("self") {
            state.token_expect_take_text_sen(")");

            Self::PubSelf
        } else if state.token_take_text_sen("super") {
            state.token_expect_take_text_sen(")");

            Self::PubSuper
        } else {
            let mut path = vec![];
            while state.not_at_end() {
                path.push(state.token_current().unwrap().into());
                state.advance();
                if state.token_take_text_sen(")") {
                    break;
                }

                if state.token_take_text_sen("::") {}
            }

            state.advance();
            Self::PubIn(path)
        };

        Some(ret)
    }
}
