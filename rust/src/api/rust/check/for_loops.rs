use crate::api::diagnostics::Diagnostics;
use crate::api::rust::check::utils::{
    check_token_print, compare_token_expr, main_tokens, mark, open_brace_after,
    split_statements, token_block,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn check_for_loop(
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
        (parse_for(&generated), parse_for(&user))
    else {
        diagnostics.record_again("Expecting a for loop", None, None);
        return;
    };
    let mut loop_ok = true;
    if expected.variable != actual.variable {
        diagnostics.record_hard(
            format!(
                "Expected loop variable `{}` but found `{}`",
                expected.variable.join(" "),
                actual.variable.join(" ")
            ),
            None,
            None,
        );
        loop_ok = false;
    }
    loop_ok &= compare_token_expr(diagnostics, &expected.iter, &actual.iter);
    mark(conditions, 0, loop_ok);

    let expected_body = split_statements(&expected.body);
    let actual_body = split_statements(&actual.body);
    mark(conditions, 1, !actual_body.is_empty());
    if actual_body.is_empty() {
        diagnostics.record_hard("The for loop body is empty", None, None);
        return;
    }
    let print_ok = expected_body
        .first()
        .zip(actual_body.first())
        .is_some_and(|(a, b)| check_token_print(diagnostics, a, b));
    mark(conditions, 2, print_ok);
}

struct ForParts {
    variable: Vec<String>,
    iter: Vec<String>,
    body: Vec<String>,
}

fn parse_for(tokens: &[String]) -> Option<ForParts> {
    if tokens.first()?.as_str() != "for" {
        return None;
    }
    let open = open_brace_after(tokens, 1)?;
    let header = &tokens[1..open];
    let in_index = header.iter().position(|token| token == "in")?;
    let (body, _) = token_block(tokens, open)?;
    Some(ForParts {
        variable: header[..in_index].to_vec(),
        iter: header[in_index + 1..].to_vec(),
        body,
    })
}
