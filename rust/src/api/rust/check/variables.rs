use crate::api::diagnostics::Diagnostics;
use crate::api::rust::check::utils::{
    check_token_let, check_token_print, compare_token_expr, main_tokens, mark,
    split_statements,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn check_variable_declaration(
    diagnostics: &mut Diagnostics,
    conditions: &mut [(bool, String)],
    generated: &str,
    user: &str,
) {
    let (Some(generated), Some(user)) =
        (main_tokens(generated), main_tokens(user))
    else {
        diagnostics.record_again("Could not find the `main` function");
        return;
    };
    let generated = split_statements(&generated);
    let user = split_statements(&user);

    for (index, expected) in generated.iter().enumerate() {
        let Some(actual) = user.get(index) else {
            diagnostics
                .record_again(format!("Missing statement {}", index + 1));
            continue;
        };
        let result = if expected.first().map(String::as_str) == Some("let") {
            check_token_let(diagnostics, expected, actual)
        } else if expected.first().map(String::as_str) == Some("println")
            || expected.first().map(String::as_str) == Some("print")
        {
            check_token_print(diagnostics, expected, actual)
        } else {
            compare_token_expr(diagnostics, expected, actual)
        };
        mark(conditions, index, result);
    }
}
