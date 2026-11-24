use crate::api::diagnostics::Diagnostics;
use crate::api::rust::check::utils::{
    check_token_let, check_token_print, compare_token_expr, main_tokens, mark,
    open_brace_after, split_statements, split_tokens, token_block,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn check_match_statement(
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
            "Missing the variable declaration before the match",
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
            "Expecting a match statement after the variable declaration",
            None,
            None,
        );
        return;
    };
    let (Some(expected), Some(actual)) =
        (parse_match(expected), parse_match(actual))
    else {
        diagnostics.record_again("Expecting a match statement", None, None);
        return;
    };
    mark(
        conditions,
        1,
        compare_token_expr(diagnostics, &expected.value, &actual.value),
    );
    if expected.arms.len() != actual.arms.len() {
        diagnostics.record_hard(
            format!(
                "Expected {} match arms but found {}",
                expected.arms.len(),
                actual.arms.len()
            ),
            None,
            None,
        );
    }
    for (index, expected_arm) in expected.arms.iter().enumerate() {
        let condition_index = 2 + index * 2;
        let Some(actual_arm) = actual.arms.get(index) else {
            mark(conditions, condition_index, false);
            mark(conditions, condition_index + 1, false);
            diagnostics.record_hard(
                format!("Missing match arm {}", index + 1),
                None,
                None,
            );
            continue;
        };
        let pattern_ok = expected_arm.pattern == actual_arm.pattern;
        if !pattern_ok {
            diagnostics.record_hard(
                format!(
                    "Expected match pattern `{}` but found `{}`",
                    expected_arm.pattern.join(" "),
                    actual_arm.pattern.join(" ")
                ),
                None,
                None,
            );
        }
        mark(conditions, condition_index, pattern_ok);
        let expected_body = split_statements(&expected_arm.body);
        let actual_body = split_statements(&actual_arm.body);
        let print_ok = expected_body
            .first()
            .zip(actual_body.first())
            .is_some_and(|(a, b)| check_token_print(diagnostics, a, b));
        if !print_ok && actual_body.is_empty() {
            diagnostics.record_hard("The match arm body is empty", None, None);
        }
        mark(conditions, condition_index + 1, print_ok);
    }
}

struct MatchArmTokens {
    pattern: Vec<String>,
    body: Vec<String>,
}

struct MatchTokens {
    value: Vec<String>,
    arms: Vec<MatchArmTokens>,
}

fn parse_match(tokens: &[String]) -> Option<MatchTokens> {
    if tokens.first()?.as_str() != "match" {
        return None;
    }
    let open = open_brace_after(tokens, 1)?;
    let (body, _) = token_block(tokens, open)?;
    let arms = split_tokens(&body, ",")
        .into_iter()
        .filter(|arm| !arm.is_empty())
        .map(|arm| {
            let arrow = arm.iter().position(|token| token == "=>")?;
            let brace =
                arm[arrow + 1..].iter().position(|token| token == "{")?
                    + arrow
                    + 1;
            let (body, after) = token_block(&arm, brace)?;
            if after != arm.len() {
                return None;
            }
            Some(MatchArmTokens { pattern: arm[..arrow].to_vec(), body })
        })
        .collect::<Option<Vec<_>>>()?;
    Some(MatchTokens { value: tokens[1..open].to_vec(), arms })
}
