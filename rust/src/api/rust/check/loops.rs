use crate::api::diagnostics::Diagnostics;
use crate::api::rust::check::utils::{
    check_token_print, main_tokens, mark, open_brace_after, split_statements,
    token_block,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn check_loop(
    diagnostics: &mut Diagnostics,
    conditions: &mut [(bool, String)],
    generated: &str,
    user: &str,
) {
    let (Some(generated), Some(user)) =
        (main_tokens(generated), main_tokens(user))
    else {
        diagnostics.record_again(
            "Could not find the `main` function",
            None,
            None,
        );
        return;
    };
    let (Some(expected), Some(actual)) =
        (parse_loop(&generated), parse_loop(&user))
    else {
        diagnostics.record_again(
            "Expecting an infinite `loop` expression",
            None,
            None,
        );
        return;
    };
    mark(conditions, 0, true);
    let expected_body = split_statements(&expected);
    let actual_body = split_statements(&actual);
    mark(conditions, 1, !actual_body.is_empty());
    if actual_body.is_empty() {
        diagnostics.record_hard("The loop body is empty", None, None);
        return;
    }
    let print_ok = expected_body
        .first()
        .zip(actual_body.first())
        .is_some_and(|(a, b)| check_token_print(diagnostics, a, b));
    mark(conditions, 2, print_ok);
    let break_ok =
        expected_body.get(1).zip(actual_body.get(1)).is_some_and(|(a, b)| {
            a == &["break".to_string()] && b == &["break".to_string()]
        });
    if !break_ok {
        diagnostics.record_hard(
            "Expected `break` after the print call",
            None,
            None,
        );
    }
    mark(conditions, 3, break_ok);
}

fn parse_loop(tokens: &[String]) -> Option<Vec<String>> {
    if tokens.first()?.as_str() != "loop" {
        return None;
    }
    let open = open_brace_after(tokens, 1)?;
    let (body, _) = token_block(tokens, open)?;
    Some(body)
}
