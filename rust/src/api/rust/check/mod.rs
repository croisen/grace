use crate::api::diagnostics::Diagnostics;
use crate::api::rust::RustConcept;
use crate::api::rust::check::for_loops::check_for_loop;
use crate::api::rust::check::functions::check_function_declaration;
use crate::api::rust::check::ifs::check_if_statement;
use crate::api::rust::check::loops::check_loop;
use crate::api::rust::check::match_statements::check_match_statement;
use crate::api::rust::check::utils::{ast, main_tokens};
use crate::api::rust::check::variables::check_variable_declaration;
use crate::api::rust::check::while_loops::check_while_loop;

mod for_loops;
mod functions;
mod ifs;
mod loops;
mod match_statements;
mod utils;
mod variables;
mod while_loops;

#[flutter_rust_bridge::frb(sync)]
pub fn rust_check(
    concept: RustConcept,
    conditions: &mut Vec<(bool, String)>,
    generated_source: &str,
    user_code: &str,
) -> Diagnostics {
    let mut diagnostics = Diagnostics { d: Vec::new() };
    for (passed, _) in conditions.iter_mut() {
        *passed = false;
    }

    if user_code.trim().is_empty() {
        diagnostics.record_again("Empty answer", None, None);
        return diagnostics;
    }

    if concept == RustConcept::FunctionDecl {
        let (user_root, user_diagnostics) = ast(user_code);
        diagnostics = user_diagnostics;
        if user_root.is_empty() {
            return diagnostics;
        }
        let (generated_root, generated_diagnostics) = ast(generated_source);
        if generated_root.is_empty() {
            diagnostics.record_again(
                "The generated drill answer could not be parsed",
                None,
                None,
            );
            return diagnostics;
        };
        if !generated_diagnostics.is_empty() {
            diagnostics.record_again(
                "The generated drill answer contains a syntax error",
                None,
                None,
            );
            return diagnostics;
        }
        check_function_declaration(
            &mut diagnostics,
            conditions,
            &generated_root,
            &user_root,
        );
        return diagnostics;
    }

    let Some(generated_main) = main_tokens(generated_source) else {
        diagnostics.record_again(
            "The generated drill has no `main` function",
            None,
            None,
        );
        return diagnostics;
    };
    let Some(user_main) = main_tokens(user_code) else {
        diagnostics.record_again("No main function found", None, None);
        return diagnostics;
    };
    if user_main.is_empty() {
        diagnostics.record_again("Empty main function", None, None);
        return diagnostics;
    }
    if generated_main.is_empty() {
        diagnostics.record_again(
            "The generated drill has an empty `main` function",
            None,
            None,
        );
        return diagnostics;
    }

    match concept {
        RustConcept::VarDecl => check_variable_declaration(
            &mut diagnostics,
            conditions,
            generated_source,
            user_code,
        ),
        RustConcept::Ifs => check_if_statement(
            &mut diagnostics,
            conditions,
            generated_source,
            user_code,
        ),
        RustConcept::ForLoop => check_for_loop(
            &mut diagnostics,
            conditions,
            generated_source,
            user_code,
        ),
        RustConcept::WhileLoop => check_while_loop(
            &mut diagnostics,
            conditions,
            generated_source,
            user_code,
        ),
        RustConcept::Loop => check_loop(
            &mut diagnostics,
            conditions,
            generated_source,
            user_code,
        ),
        RustConcept::Match => check_match_statement(
            &mut diagnostics,
            conditions,
            generated_source,
            user_code,
        ),
        RustConcept::FunctionDecl => unreachable!(),
    }

    diagnostics
}
