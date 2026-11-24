use crate::api::diagnostics::Diagnostics;
use crate::api::rust::check::utils::{
    check_token_print, compare_token_condition, main_tokens, mark,
    open_brace_after, split_statements, token_block,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn check_if_statement(
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
    let (Some(g_if), Some(u_if)) = (parse_if(&generated), parse_if(&user))
    else {
        diagnostics.record_again("Expecting an if statement");
        return;
    };

    mark(
        conditions,
        0,
        compare_token_condition(diagnostics, &g_if.condition, &u_if.condition),
    );
    let expected_then = split_statements(&g_if.then_body);
    let actual_then = split_statements(&u_if.then_body);
    mark(conditions, 1, !actual_then.is_empty());
    if actual_then.is_empty() {
        diagnostics.record_hard("The if block is empty");
    } else {
        mark(
            conditions,
            2,
            expected_then
                .first()
                .zip(actual_then.first())
                .is_some_and(|(a, b)| check_token_print(diagnostics, a, b)),
        );
    }

    let Some(expected_else) = g_if.else_body else {
        diagnostics.record_again("The generated drill has no else block");
        return;
    };
    let Some(actual_else) = u_if.else_body else {
        diagnostics.record_again("Expected an else block");
        return;
    };
    mark(conditions, 3, true);
    let expected_else = split_statements(&expected_else);
    let actual_else = split_statements(&actual_else);
    if actual_else.is_empty() {
        diagnostics.record_hard("The else block is empty");
        mark(conditions, 4, false);
    } else {
        let print_ok = expected_else
            .first()
            .zip(actual_else.first())
            .is_some_and(|(a, b)| check_token_print(diagnostics, a, b));
        mark(conditions, 4, print_ok);
    }
}

struct IfParts {
    condition: Vec<String>,
    then_body: Vec<String>,
    else_body: Option<Vec<String>>,
}

fn parse_if(tokens: &[String]) -> Option<IfParts> {
    if tokens.first()?.as_str() != "if" {
        return None;
    }
    let open = open_brace_after(tokens, 1)?;
    let (then_body, after_then) = token_block(tokens, open)?;
    let else_body =
        if tokens.get(after_then).map(String::as_str) == Some("else") {
            let else_open = after_then + 1;
            if tokens.get(else_open).map(String::as_str) != Some("{") {
                return None;
            }
            let (body, _) = token_block(tokens, else_open)?;
            Some(body)
        } else {
            None
        };
    Some(IfParts { condition: tokens[1..open].to_vec(), then_body, else_body })
}
