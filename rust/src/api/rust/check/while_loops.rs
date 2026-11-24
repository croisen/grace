use crate::api::diagnostics::Diagnostics;
use crate::api::rust::check::utils::{
    check_token_let, check_token_print, compare_token_condition,
    compare_token_expr, main_tokens, mark, open_brace_after, split_statements,
    token_block,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn check_while_loop(
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
    let (g_statements, u_statements) =
        (split_statements(&generated), split_statements(&user));
    let (Some(g_decl), Some(u_decl)) =
        (g_statements.first(), u_statements.first())
    else {
        diagnostics.record_hard(
            "Missing the loop variable declaration",
            None,
            None,
        );
        return;
    };
    mark(conditions, 0, check_token_let(diagnostics, g_decl, u_decl));
    let (Some(expected), Some(actual)) =
        (g_statements.get(1), u_statements.get(1))
    else {
        diagnostics.record_again(
            "Expecting a while loop after the variable declaration",
            None,
            None,
        );
        return;
    };
    let (Some(expected), Some(actual)) =
        (parse_while(expected), parse_while(actual))
    else {
        diagnostics.record_again(
            "Expecting a while loop after the variable declaration",
            None,
            None,
        );
        return;
    };
    mark(
        conditions,
        1,
        compare_token_condition(
            diagnostics,
            &expected.condition,
            &actual.condition,
        ),
    );
    let expected_body = split_statements(&expected.body);
    let actual_body = split_statements(&actual.body);
    mark(conditions, 2, !actual_body.is_empty());
    if actual_body.is_empty() {
        diagnostics.record_hard("The while loop body is empty", None, None);
        return;
    }
    let print_ok = expected_body
        .first()
        .zip(actual_body.first())
        .is_some_and(|(a, b)| check_token_print(diagnostics, a, b));
    mark(conditions, 3, print_ok);
    let update_ok = expected_body
        .get(1)
        .zip(actual_body.get(1))
        .is_some_and(|(a, b)| compare_token_expr(diagnostics, a, b));
    if !update_ok {
        diagnostics.record_hard(
            "Missing or incorrect update statement in the while loop",
            None,
            None,
        );
    }
    mark(conditions, 4, update_ok);
}

struct WhileParts {
    condition: Vec<String>,
    body: Vec<String>,
}

fn parse_while(tokens: &[String]) -> Option<WhileParts> {
    if tokens.first()?.as_str() != "while" {
        return None;
    }
    let open = open_brace_after(tokens, 1)?;
    let (body, _) = token_block(tokens, open)?;
    Some(WhileParts { condition: tokens[1..open].to_vec(), body })
}
