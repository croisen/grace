use crate::core::parser::output::ParserOutput;
use crate::core::parser::state::ParserState;
use crate::langs::rust::ast::expression::{RustBlock, RustExpression};
use crate::langs::rust::ast::item::RustItem;
use crate::langs::rust::ast::item::function::RustFunction;
use crate::langs::rust::ast::statement::RustStatement;
use crate::langs::rust::ast::r#type::RustType;
use crate::langs::rust::ast::r#type::generics::{
    RustGeneric, RustGenericParam,
};
use crate::langs::rust::ast::r#type::lifetime::RustLifetime;
use crate::langs::rust::ast::r#type::parameter::RustParameter;
use crate::langs::rust::ast::visibility::RustVisibility;
use crate::langs::rust::lexer::RustTokenKind;

pub mod expression;
pub mod item;
pub mod pattern;
pub mod statement;
pub mod r#type;
pub mod visibility;

pub type RustParserState<'a> = ParserState<'a, RustTokenKind, RustAst>;

#[derive(Debug, Clone)]
pub struct RustAst {
    pub visibility: RustVisibility,
    pub item: RustItem,
}

impl RustAst {
    pub fn new<'a>(source: &'a str) -> ParserOutput<Self> {
        let mut state = ParserState::<'a, RustTokenKind, Self>::new(source);
        while state.not_at_end() {
            let visibility = RustVisibility::new(&mut state)
                .unwrap_or(RustVisibility::PubSelf);
            let mut is_async = false;
            let mut is_unsafe = false;
            if state.token_take_text_sen("async") {
                is_async = true;
            }
            if state.token_take_text_sen("unsafe") {
                is_unsafe = true;
            }
            if !state.token_take_text_sen("fn") {
                state.advance();
                continue;
            }
            let name =
                state.token_current().map(Into::into).unwrap_or_default();
            state.advance();
            let mut generic_params = Vec::new();
            if state.token_take_text_sen("<") {
                while state.not_at_end() && !state.token_eat_text_sen(">") {
                    if matches!(
                        state.token_kind(),
                        Some(
                            RustTokenKind::Lifetime
                                | RustTokenKind::StaticLifetime
                        )
                    ) {
                        let lifetime = state.token_current().unwrap().into();
                        generic_params.push(RustGenericParam::Lifetime(
                            RustLifetime::new(lifetime),
                        ));
                        state.advance();
                    } else {
                        let ty: crate::core::parser::pratt::Token2 = state
                            .token_current()
                            .map(Into::into)
                            .unwrap_or_default();
                        generic_params.push(RustGenericParam::Type(
                            ty,
                            vec![],
                            None,
                        ));
                        state.advance();
                    }
                    if state.token_take_text_sen(",") {
                    } else {
                        break;
                    }
                }
                state.token_take_text_sen(">");
            }
            state.token_expect_take_text_sen("(");
            let mut params = Vec::new();
            while state.not_at_end() && !state.token_eat_text_sen(")") {
                let param_name =
                    state.token_current().map(Into::into).unwrap_or_default();
                state.advance();
                state.token_expect_take_text_sen(":");
                let (ty, is_mut) = parse_type(&mut state);
                params.push(RustParameter {
                    name: param_name,
                    r#type: ty,
                    r#mut: is_mut,
                });
                if state.token_take_text_sen(",") {
                } else {
                    break;
                }
            }
            state.token_take_text_sen(")");
            let r#return = if state.token_take_text_sen("->") {
                Some(parse_type(&mut state).0)
            } else {
                None
            };
            while state.not_at_end() && !state.token_eat_text_sen("{") {
                state.advance();
            }
            let body = if state.token_take_text_sen("{") {
                parse_block(&mut state)
            } else {
                RustBlock { statements: vec![], level: 0 }
            };
            let function = RustFunction {
                name,
                generics: if generic_params.is_empty() {
                    vec![]
                } else {
                    vec![RustGeneric {
                        params: generic_params,
                        r#where: vec![],
                    }]
                },
                params,
                r#return,
                r#async: is_async,
                r#unsafe: is_unsafe,
                r#extern: false,
                r#const: false,
                body,
            };
            state.root_push(Self {
                visibility,
                item: RustItem::Function(function),
            });
        }
        state.into()
    }
}

fn parse_type(state: &mut RustParserState<'_>) -> (RustType, bool) {
    if state.token_take_text_sen("&") {
        let lifetime = if matches!(
            state.token_kind(),
            Some(RustTokenKind::Lifetime | RustTokenKind::StaticLifetime)
        ) {
            let token = state.token_current().unwrap().into();
            Some(RustLifetime::new(token))
        } else {
            None
        };
        let is_mut =
            if state.token_take_text_sen("mut") { true } else { false };
        let inner = parse_type(state).0;
        return (
            RustType::Reference(lifetime, is_mut, Box::new(inner)),
            is_mut,
        );
    }
    let mut path =
        vec![state.token_current().map(Into::into).unwrap_or_default()];
    state.advance();
    while state.token_take_text_sen("::") {
        path.push(state.token_current().map(Into::into).unwrap_or_default());
        state.advance();
    }
    (RustType::Path(path), false)
}

fn parse_block(state: &mut RustParserState<'_>) -> RustBlock {
    let mut statements = Vec::new();
    while state.not_at_end() && !state.token_eat_text_sen("}") {
        if state.token_take_text_sen("let") {
            let is_mut =
                if state.token_take_text_sen("mut") { true } else { false };
            let name =
                state.token_current().map(Into::into).unwrap_or_default();
            state.advance();
            let ty = if state.token_take_text_sen(":") {
                Some(parse_type(state).0)
            } else {
                None
            };
            let init = if state.token_take_text_sen("=") {
                Some(RustExpression::new(state))
            } else {
                None
            };
            state.token_take_text_sen(";");
            statements.push(RustStatement::Let {
                mutable: is_mut,
                name,
                r#type: ty,
                init,
            });
        } else if state.token_take_text_sen("return") {
            let value = if state.token_take_text_sen(";")
                || state.token_eat_text_sen("}")
            {
                None
            } else {
                Some(RustExpression::new(state))
            };
            state.token_take_text_sen(";");
            statements.push(RustStatement::Return(value));
        } else if state.token_take_text_sen("break") {
            state.token_take_text_sen(";");
            statements.push(RustStatement::Break);
        } else if state.token_take_text_sen("continue") {
            state.token_take_text_sen(";");
            statements.push(RustStatement::Continue);
        } else {
            let expr = RustExpression::new(state);
            let terminated = state.token_take_text_sen(";");
            statements.push(RustStatement::Expression {
                expression: expr,
                has_semicolon: terminated,
            });
        }
    }
    state.token_take_text_sen("}");
    RustBlock { statements, level: 0 }
}
