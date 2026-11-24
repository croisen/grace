use oak_core::{Builder, ParseSession, SourceText};
use oak_rust::{RustBuilder, RustLanguage, RustRoot};

use crate::api::diagnostics::Diagnostics;
use crate::api::rust::RustConcept;
use crate::api::rust::check::for_loops::check_for_loop;
use crate::api::rust::check::functions::check_function_declaration;
use crate::api::rust::check::ifs::check_if_statement;
use crate::api::rust::check::loops::check_loop;
use crate::api::rust::check::match_statements::check_match_statement;
use crate::api::rust::check::utils::main_tokens;
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
    let mut diagnostics = Diagnostics::new(user_code, Vec::new());
    for (passed, _) in conditions.iter_mut() {
        *passed = false;
    }

    if user_code.trim().is_empty() {
        diagnostics.record_again("Empty answer");
        return diagnostics;
    }

    if concept == RustConcept::FunctionDecl {
        let (user_root, user_diagnostics) = ast(user_code);
        diagnostics = user_diagnostics;
        let Some(user_root) = user_root else {
            return diagnostics;
        };
        let (generated_root, generated_diagnostics) = ast(generated_source);
        let Some(generated_root) = generated_root else {
            diagnostics
                .record_again("The generated drill answer could not be parsed");
            return diagnostics;
        };
        if !generated_diagnostics.is_empty() {
            diagnostics.record_again(
                "The generated drill answer contains a syntax error",
            );
            return diagnostics;
        }
        check_function_declaration(
            &mut diagnostics,
            conditions,
            &generated_root.items,
            &user_root.items,
        );
        return diagnostics;
    }

    let Some(generated_main) = main_tokens(generated_source) else {
        diagnostics.record_again("The generated drill has no `main` function");
        return diagnostics;
    };
    let Some(user_main) = main_tokens(user_code) else {
        diagnostics.record_again("No main function found");
        return diagnostics;
    };
    if user_main.is_empty() {
        diagnostics.record_again("Empty main function");
        return diagnostics;
    }
    if generated_main.is_empty() {
        diagnostics
            .record_again("The generated drill has an empty `main` function");
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

fn ast(source: &str) -> (Option<RustRoot>, Diagnostics) {
    let lang = RustLanguage::new();
    let b = RustBuilder::new(&lang);
    let mut c = ParseSession::default();

    let mut r = None;
    let s = SourceText::new(source);
    let mut a = b.build(&s, &[], &mut c);

    match a.result {
        Ok(root) => r = Some(root),
        Err(e) => a.diagnostics.push(e),
    }

    (r, Diagnostics::new(&source, a.diagnostics))
}
