use crate::core::parser::output::ParserOutput;
use crate::core::parser::pratt::Token2;
use crate::core::parser::state::ParserState;
use crate::langs::java::ast::class::JavaClass;
use crate::langs::java::ast::constructor::JavaConstructor;
use crate::langs::java::ast::expression::JavaExpression;
use crate::langs::java::ast::field::JavaField;
use crate::langs::java::ast::member::JavaMember;
use crate::langs::java::ast::method::JavaMethod;
use crate::langs::java::ast::packages::{JavaImport, JavaPackage};
use crate::langs::java::ast::parameter::JavaParameter;
use crate::langs::java::ast::statement::JavaStatement;
use crate::langs::java::lexer::JavaTokenKind;

pub mod annotation;
pub mod class;
pub mod constructor;
pub mod r#enum;
pub mod expression;
pub mod field;
pub mod generic_type;
pub mod interface;
pub mod member;
pub mod method;
pub mod packages;
pub mod parameter;
pub mod record;
pub mod statement;
pub mod r#struct;
pub mod type_parameter;

pub type JavaParserState<'a> = ParserState<'a, JavaTokenKind, JavaAst>;

#[derive(Debug, Clone, Default)]
pub struct JavaAst {
    pub package: Option<JavaPackage>,
    pub imports: Vec<JavaImport>,
    pub classes: Vec<JavaClass>,
}

impl JavaAst {
    pub fn new(source: &str) -> ParserOutput<Self> {
        let mut state = JavaParserState::new(source);
        let mut ast = Self::default();
        while state.not_at_end() {
            if state.token_take_text_sen("package") {
                let name = collect_path(&mut state, ";");
                state.token_expect_text_sen(";");
                state.advance();
                ast.package = Some(JavaPackage { name });
            } else if state.token_take_text_sen("import") {
                let is_static = if state.token_take_text_sen("static") {
                    true
                } else {
                    false
                };
                let path = collect_path(&mut state, ";");
                state.token_expect_text_sen(";");
                state.advance();
                ast.imports.push(JavaImport { path, is_static });
            } else if state.token_take_text_sen("class") {
                let name =
                    state.token_current().map(Into::into).unwrap_or_default();
                state.advance();
                while state.not_at_end() && !state.token_eat_text_sen("{") {
                    state.advance();
                }
                if state.token_take_text_sen("{") {}
                let mut class = JavaClass {
                    name,
                    type_parameters: vec![],
                    modifiers: vec![],
                    annotations: vec![],
                    extends: None,
                    implements: vec![],
                    members: vec![],
                };
                while state.not_at_end() && !state.token_eat_text_sen("}") {
                    if let Some(member) = parse_member(&mut state, &class.name)
                    {
                        class.members.push(member);
                    }
                }
                if state.token_take_text_sen("}") {}
                ast.classes.push(class);
            } else {
                state.advance();
            }
        }
        state.root_push(ast);
        state.into()
    }
}

fn parse_member(
    state: &mut JavaParserState<'_>,
    class_name: &Token2,
) -> Option<JavaMember> {
    let mut modifiers: Vec<Token2> = Vec::new();
    while state.token_text().is_some_and(|t| {
        matches!(
            t,
            "public"
                | "private"
                | "protected"
                | "static"
                | "final"
                | "abstract"
                | "synchronized"
                | "native"
                | "transient"
                | "volatile"
                | "strictfp"
        )
    }) {
        modifiers.push(state.token_current()?.into());
        state.advance();
    }
    if !state.not_at_end() || state.token_eat_text_sen("}") {
        return None;
    }
    let first: Token2 = state.token_current()?.into();
    state.advance();
    let second: Token2 = state.token_current()?.into();
    state.advance();

    let is_callable = state.token_eat_text_sen("(");
    if is_callable {
        let name = second;
        state.advance();
        let mut parameters = Vec::new();
        while state.not_at_end() && !state.token_eat_text_sen(")") {
            let mut ty: Token2 = state.token_current()?.into();
            state.advance();
            let is_varargs = state.token_eat_text_sen("...");
            if is_varargs {
                ty.text.push_str("...");
                state.advance();
            }
            while state.token_eat_text_sen("[") {
                ty.text.push_str("[]");
                state.advance();
                if state.token_take_text_sen("]") {}
            }
            let param_name: Token2 = state.token_current()?.into();
            state.advance();
            parameters.push(JavaParameter {
                name: param_name,
                r#type: ty,
                is_varargs,
            });
            if state.token_take_text_sen(",") {
            } else {
                break;
            }
        }
        if state.token_take_text_sen(")") {}
        while state.not_at_end() && !state.token_eat_texts_sen(&["{", ";"]) {
            state.advance();
        }
        let mut body = Vec::new();
        if state.token_take_text_sen("{") {
            body = parse_block(state);
            state.token_take_text_sen("}");
        } else if state.token_eat_text_sen(";") {
            state.advance();
        }
        if name.text == class_name.text {
            Some(JavaMember::Constructor(JavaConstructor {
                modifiers,
                annotations: vec![],
                name,
                parameters,
                body,
            }))
        } else {
            Some(JavaMember::Method(JavaMethod {
                name,
                type_parameters: vec![],
                modifiers: modifiers.clone(),
                annotations: vec![],
                return_type: first,
                parameters,
                body,
                throws: vec![],
                is_static: modifiers.iter().any(|m| m.text == "static"),
            }))
        }
    } else {
        let name = second;
        while state.not_at_end() && !state.token_eat_texts_sen(&[";", "}"]) {
            state.advance();
        }
        if state.token_eat_text_sen(";") {
            state.advance();
        }
        Some(JavaMember::Field(JavaField {
            name,
            r#type: first,
            modifiers,
            annotations: vec![],
        }))
    }
}

fn parse_block(state: &mut JavaParserState<'_>) -> Vec<JavaStatement> {
    let mut statements = Vec::new();
    while state.not_at_end() && !state.token_eat_text_sen("}") {
        if state.token_eat_text_sen(";") {
            state.advance();
            continue;
        }
        if let Some(statement) = parse_statement(state) {
            statements.push(statement);
        }
    }
    statements
}

fn parse_statement(state: &mut JavaParserState<'_>) -> Option<JavaStatement> {
    if state.token_take_text_sen("{") {
        let nested = parse_block(state);
        if state.token_take_text_sen("}") {}
        return Some(JavaStatement::Block(nested));
    }
    if state.token_take_text_sen("if") {
        state.token_take_text_sen("(");
        let condition = JavaExpression::new(state);
        if state.token_take_text_sen(")") {}
        let then = Box::new(
            parse_statement(state).unwrap_or(JavaStatement::Block(vec![])),
        );
        let r#else = if state.token_take_text_sen("else") {
            Some(Box::new(
                parse_statement(state).unwrap_or(JavaStatement::Block(vec![])),
            ))
        } else {
            None
        };
        return Some(JavaStatement::If { condition, then, r#else });
    }
    if state.token_take_text_sen("while") {
        state.token_take_text_sen("(");
        let condition = JavaExpression::new(state);
        if state.token_take_text_sen(")") {}
        let body = Box::new(
            parse_statement(state).unwrap_or(JavaStatement::Block(vec![])),
        );
        return Some(JavaStatement::While { condition, body });
    }
    if state.token_take_text_sen("do") {
        let body = Box::new(
            parse_statement(state).unwrap_or(JavaStatement::Block(vec![])),
        );
        if state.token_take_text_sen("while") {}
        state.token_take_text_sen("(");
        let condition = JavaExpression::new(state);
        if state.token_take_text_sen(")") {}
        if state.token_eat_text_sen(";") {
            state.advance();
        }
        return Some(JavaStatement::DoWhile { condition, body });
    }
    if state.token_take_text_sen("for") {
        state.token_take_text_sen("(");
        if state.token_kind() == Some(&JavaTokenKind::Identifier)
            && state.token_peek_at_kind(1) == Some(&JavaTokenKind::Identifier)
            && state.token_peek_at_text(2) == Some(":")
        {
            let r#type: Token2 =
                state.token_current().map(Into::into).unwrap_or_default();
            state.advance();
            let name: Token2 =
                state.token_current().map(Into::into).unwrap_or_default();
            state.advance();
            state.advance(); // colon
            let iter = JavaExpression::new(state);
            if state.token_take_text_sen(")") {}
            let body = Box::new(
                parse_statement(state).unwrap_or(JavaStatement::Block(vec![])),
            );
            return Some(JavaStatement::ForEach { r#type, name, iter, body });
        }
        let init = if state.token_eat_text_sen(";") {
            state.advance();
            None
        } else {
            parse_statement(state).map(Box::new)
        };
        let condition = if state.token_eat_text_sen(";") {
            None
        } else {
            Some(JavaExpression::new(state))
        };
        if state.token_eat_text_sen(";") {
            state.advance();
        }
        let update = if state.token_eat_text_sen(")") {
            None
        } else {
            Some(JavaExpression::new(state))
        };
        if state.token_take_text_sen(")") {}
        let body = Box::new(
            parse_statement(state).unwrap_or(JavaStatement::Block(vec![])),
        );
        return Some(JavaStatement::For { init, condition, update, body });
    }
    if state.token_take_text_sen("switch") {
        state.token_take_text_sen("(");
        let selector = JavaExpression::new(state);
        if state.token_take_text_sen(")") {}
        state.token_take_text_sen("{");
        let mut cases = Vec::new();
        let mut default = Vec::new();
        while state.not_at_end() && !state.token_eat_text_sen("}") {
            let is_default = state.token_eat_text_sen("default");
            if is_default || state.token_eat_text_sen("case") {
                state.advance();
                let label = if is_default {
                    Token2::default()
                } else {
                    state.token_current().map(Into::into).unwrap_or_default()
                };
                if !is_default {
                    state.advance();
                }
                if state.token_take_text_sen(":") {}
                let mut body = Vec::new();
                while state.not_at_end()
                    && !state.token_eat_texts_sen(&["case", "default", "}"])
                {
                    if let Some(statement) = parse_statement(state) {
                        body.push(statement);
                    }
                }
                if is_default {
                    default.extend(body);
                } else {
                    cases.push(
                        crate::langs::java::ast::statement::JavaSwitchCase {
                            label,
                            body,
                        },
                    );
                }
            } else {
                state.advance();
            }
        }
        if state.token_take_text_sen("}") {}
        return Some(JavaStatement::Switch { selector, cases, default });
    }
    if state.token_take_text_sen("break") {
        if state.token_eat_text_sen(";") {
            state.advance();
        }
        return Some(JavaStatement::Break);
    }
    if state.token_take_text_sen("continue") {
        if state.token_eat_text_sen(";") {
            state.advance();
        }
        return Some(JavaStatement::Continue);
    }
    if state.token_take_text_sen("return") {
        let value = if state.token_eat_text_sen(";") {
            None
        } else {
            Some(JavaExpression::new(state))
        };
        if state.token_eat_text_sen(";") {
            state.advance();
        }
        return Some(JavaStatement::Return(value));
    }
    if state.token_take_text_sen("throw") {
        let value = JavaExpression::new(state);
        if state.token_eat_text_sen(";") {
            state.advance();
        }
        return Some(JavaStatement::Throw(value));
    }
    let looks_like_declaration = state.token_kind()
        == Some(&JavaTokenKind::Identifier)
        && state.token_peek_at_kind(1) == Some(&JavaTokenKind::Identifier);
    if looks_like_declaration {
        let ty: Token2 =
            state.token_current().map(Into::into).unwrap_or_default();
        state.advance();
        let name: Token2 =
            state.token_current().map(Into::into).unwrap_or_default();
        state.advance();
        let init = if state.token_take_text_sen("=") {
            Some(JavaExpression::new(state))
        } else {
            None
        };
        while state.not_at_end() && !state.token_eat_texts_sen(&[";", "}"]) {
            state.advance();
        }
        if state.token_eat_text_sen(";") {
            state.advance();
        }
        return Some(JavaStatement::LocalVariable { name, r#type: ty, init });
    }
    let expression = JavaExpression::new(state);
    if state.token_eat_text_sen(";") {
        state.advance();
    }
    Some(JavaStatement::Expression(expression))
}

fn collect_path(state: &mut JavaParserState<'_>, end: &str) -> Vec<Token2> {
    let mut path = Vec::new();
    while state.not_at_end() && !state.token_eat_text_sen(end) {
        if let Some(token) = state.token_current() {
            path.push(token.into());
        }
        state.advance();
    }
    path
}
