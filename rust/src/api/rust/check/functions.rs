use crate::api::diagnostics::Diagnostics;
use crate::api::rust::check::utils::{
    check_let, check_print, compare_expr, find_function,
    find_non_main_function, mark, rust_type_location, rust_type_matches,
    statement_location,
};
use crate::langs::rust::ast::RustAst;
use crate::langs::rust::ast::expression::RustExpression;
use crate::langs::rust::ast::statement::RustStatement;

#[flutter_rust_bridge::frb(ignore)]
pub fn check_function_declaration(
    diagnostics: &mut Diagnostics,
    conditions: &mut [(bool, String)],
    generated_items: &[RustAst],
    user_items: &[RustAst],
) {
    let Some(expected) = find_non_main_function(generated_items) else {
        diagnostics.record_again(
            "The generated drill has no function declaration",
            None,
            None,
        );
        return;
    };
    let Some(actual) = find_non_main_function(user_items) else {
        diagnostics.record_again(
            "No function declaration found besides `main`",
            None,
            None,
        );
        return;
    };

    let mut signature_ok = true;
    if expected.name.text != actual.name.text {
        diagnostics.record_hard(
            format!(
                "Expected function `{}` but found `{}`",
                expected.name.text, actual.name.text
            ),
            Some(actual.name.line),
            Some(actual.name.col),
        );
        signature_ok = false;
    }
    if expected.params.len() != actual.params.len() {
        let token = actual
            .params
            .get(expected.params.len())
            .or_else(|| actual.params.last())
            .map(|parameter| &parameter.name)
            .unwrap_or(&actual.name);
        diagnostics.record_hard(
            format!(
                "Expected {} parameter(s) but found {}",
                expected.params.len(),
                actual.params.len()
            ),
            Some(token.line),
            Some(token.col),
        );
        signature_ok = false;
    }
    for (expected_param, actual_param) in
        expected.params.iter().zip(&actual.params)
    {
        if expected_param.name.text != actual_param.name.text {
            diagnostics.record_hard(
                format!(
                    "Expected parameter `{}` but found `{}`",
                    expected_param.name.text, actual_param.name.text
                ),
                Some(actual_param.name.line),
                Some(actual_param.name.col),
            );
            signature_ok = false;
        }
        if !rust_type_matches(
            Some(&expected_param.r#type),
            Some(&actual_param.r#type),
        ) {
            let (line, col) = rust_type_location(&actual_param.r#type);
            diagnostics.record_hard(
                format!(
                    "Parameter `{}` has the wrong type",
                    expected_param.name.text
                ),
                line,
                col,
            );
            signature_ok = false;
        }
    }
    if !rust_type_matches(expected.r#return.as_ref(), actual.r#return.as_ref())
    {
        let (line, col) = actual
            .r#return
            .as_ref()
            .map(rust_type_location)
            .unwrap_or((Some(actual.name.line), Some(actual.name.col)));
        diagnostics.record_hard(
            "The function has the wrong return type",
            line,
            col,
        );
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
            let (line, col) = actual
                .body
                .statements
                .last()
                .and_then(statement_location)
                .unwrap_or((Some(actual.name.line), Some(actual.name.col)));
            diagnostics.record_hard(
                "The function must return a result",
                line,
                col,
            );
            false
        }
        (None, _) => {
            diagnostics.record_hard(
                "The generated function has no result expression",
                Some(actual.name.line),
                Some(actual.name.col),
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
        diagnostics.record_hard(
            "The answer must include a `main` function",
            None,
            None,
        );
        return;
    };
    let expected_main = &expected_main.body.statements;
    let actual_main = &actual_main.body.statements;
    let call_ok = match (expected_main.first(), actual_main.first()) {
        (Some(expected), Some(actual)) => {
            check_let(diagnostics, expected, actual)
        }
        _ => {
            diagnostics.record_hard(
                "Expected a call to the declared function in `main`",
                None,
                None,
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
            diagnostics.record_hard(
                "Expected the result to be printed in `main`",
                None,
                None,
            );
            false
        }
    };
    mark(conditions, 4, print_ok);
}

fn function_result(
    function: &crate::langs::rust::ast::item::function::RustFunction,
) -> Option<&RustExpression> {
    match function.body.statements.last()? {
        RustStatement::Expression { expression, has_semicolon: false } => {
            Some(expression)
        }
        RustStatement::Return(Some(expr)) => Some(expr),
        _ => None,
    }
}
