use oak_rust::ast::{Expr, Item, Statement};

use crate::api::diagnostics::Diagnostics;
use crate::api::rust::check::utils::{
    check_let, check_print, compare_expr, find_function,
    find_non_main_function, mark,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn check_function_declaration(
    diagnostics: &mut Diagnostics,
    conditions: &mut [(bool, String)],
    generated_items: &[Item],
    user_items: &[Item],
) {
    let Some(expected) = find_non_main_function(generated_items) else {
        diagnostics
            .record_again("The generated drill has no function declaration");
        return;
    };
    let Some(actual) = find_non_main_function(user_items) else {
        diagnostics
            .record_again("No function declaration found besides `main`");
        return;
    };

    let mut signature_ok = true;
    if expected.name.name != actual.name.name {
        diagnostics.record_hard(format!(
            "Expected function `{}` but found `{}`",
            expected.name.name, actual.name.name
        ));
        signature_ok = false;
    }
    if expected.params.len() != actual.params.len() {
        diagnostics.record_hard(format!(
            "Expected {} parameter(s) but found {}",
            expected.params.len(),
            actual.params.len()
        ));
        signature_ok = false;
    }
    for (expected_param, actual_param) in
        expected.params.iter().zip(&actual.params)
    {
        if expected_param.name.name != actual_param.name.name {
            diagnostics.record_hard(format!(
                "Expected parameter `{}` but found `{}`",
                expected_param.name.name, actual_param.name.name
            ));
            signature_ok = false;
        }
        if expected_param.ty != actual_param.ty {
            diagnostics.record_hard(format!(
                "Parameter `{}` has the wrong type",
                expected_param.name.name
            ));
            signature_ok = false;
        }
    }
    if expected.return_type != actual.return_type {
        diagnostics.record_hard("The function has the wrong return type");
        signature_ok = false;
    }
    mark(conditions, 0, signature_ok);

    let expected_result = function_result(expected);
    let actual_result = function_result(actual);
    let result_ok = match (expected_result, actual_result) {
        (Some(expected), Some(actual)) => {
            compare_expr(diagnostics, expected, actual)
        }
        (_, None) => {
            diagnostics.record_again("The function must return a result");
            false
        }
        (None, _) => {
            diagnostics.record_again(
                "The generated function has no result expression",
            );
            false
        }
    };
    mark(conditions, 1, !actual.body.statements.is_empty());
    mark(conditions, 2, result_ok);

    let (Some(expected_main), Some(actual_main)) = (
        find_function(generated_items, "main"),
        find_function(user_items, "main"),
    ) else {
        diagnostics.record_again("The answer must include a `main` function");
        return;
    };
    let expected_main = &expected_main.body.statements;
    let actual_main = &actual_main.body.statements;
    let call_ok = match (expected_main.first(), actual_main.first()) {
        (Some(expected), Some(actual)) => {
            check_let(diagnostics, expected, actual)
        }
        _ => {
            diagnostics.record_again(
                "Expected a call to the declared function in `main`",
            );
            false
        }
    };
    mark(conditions, 3, call_ok);
    let print_ok = match (expected_main.get(1), actual_main.get(1)) {
        (Some(expected), Some(actual)) => {
            check_print(diagnostics, expected, actual)
        }
        _ => {
            diagnostics
                .record_again("Expected the result to be printed in `main`");
            false
        }
    };
    mark(conditions, 4, print_ok);
}

fn function_result(function: &oak_rust::ast::Function) -> Option<&Expr> {
    match function.body.statements.last()? {
        Statement::ExprStmt { expr, semi: false, .. } => Some(expr),
        Statement::Return { expr: Some(expr), .. } => Some(expr),
        _ => None,
    }
}
