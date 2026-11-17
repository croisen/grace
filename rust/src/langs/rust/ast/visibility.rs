use crate::langs::rust::ast::RustParserState;

#[derive(Debug, Clone)]
pub enum RustVisibility {
    Pub,
    PubCrate,
    PubSelf,
    PubSuper,
    PubIn(Vec<String>),
}

impl RustVisibility {
    pub fn new(state: &mut RustParserState) -> Option<Self> {
        if !state.token_eat_text_sen("pub") {
            return None;
        }

        state.advance();
        if !state.token_eat_text_sen("(") {
            return Some(Self::Pub);
        }

        state.advance();
        let ret = if state.token_eat_text_sen("crate") {
            state.advance();
            if state.token_expect_text_sen(")") {
                state.advance();
            }

            Self::PubCrate
        } else if state.token_eat_text_sen("self") {
            state.advance();
            if state.token_expect_text_sen(")") {
                state.advance();
            }

            Self::PubSelf
        } else if state.token_eat_text_sen("super") {
            state.advance();
            if state.token_expect_text_sen(")") {
                state.advance();
            }

            Self::PubSuper
        } else {
            let mut path = vec![];
            while state.not_at_end() {
                path.push(state.token_text().unwrap().to_string());
                state.advance();
                if state.token_eat_text_sen(")") {
                    break;
                }

                if state.token_expect_text_sen("::") {
                    state.advance();
                }
            }

            state.advance();
            Self::PubIn(path)
        };

        Some(ret)
    }
}
