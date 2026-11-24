use super::utils::{
    ast, check_let, check_print, compare_expr as compare_expression,
    find_function, mark, statement_location,
};
use crate::api::diagnostics::Diagnostics;
use crate::langs::rust::ast::expression::RustExpression;
use crate::langs::rust::ast::statement::RustStatement;

#[flutter_rust_bridge::frb(ignore)]
pub fn check_variable_declaration(
    diagnostics: &mut Diagnostics,
    conditions: &mut [(bool, String)],
    generated: &str,
    user: &str,
) {
    let (generated_items, generated_errors) = ast(generated);
    let (user_items, user_errors) = ast(user);
    diagnostics.d.extend(user_errors.d);
    if !diagnostics.is_empty() {
        return;
    }
    if !generated_errors.is_empty() {
        diagnostics.record_again(
            "The generated drill answer could not be parsed",
            None,
            None,
        );
        return;
    }

    let (Some(expected), Some(actual)) = (
        find_function(&generated_items, "main"),
        find_function(&user_items, "main"),
    ) else {
        diagnostics.record_again(
            "Could not find the `main` function",
            None,
            None,
        );
        return;
    };

    for (index, expected) in expected.body.statements.iter().enumerate() {
        let Some(actual) = actual.body.statements.get(index) else {
            diagnostics.record_hard(
                format!("Missing statement {}", index + 1),
                None,
                None,
            );
            mark(conditions, index, false);
            continue;
        };
        let passed = match (expected, actual) {
            (RustStatement::Let { .. }, RustStatement::Let { .. }) => {
                check_let(diagnostics, expected, actual)
            }
            (
                RustStatement::Expression { .. },
                RustStatement::Expression { .. },
            ) if is_print(expected) => {
                check_print(diagnostics, expected, actual)
            }
            (
                RustStatement::Expression { expression: a, .. },
                RustStatement::Expression { expression: b, .. },
            ) => compare_expression(diagnostics, a, b),
            _ => {
                let (line, col) =
                    statement_location(actual).unwrap_or((None, None));
                diagnostics.record_hard(
                    "The statement does not match the drill",
                    line,
                    col,
                );
                false
            }
        };
        mark(conditions, index, passed);
    }
}

fn is_print(statement: &RustStatement) -> bool {
    matches!(
        statement,
        RustStatement::Expression {
            expression: RustExpression::FunctionCall { callee, .. },
            ..
        } if matches!(callee.as_ref(), RustExpression::Identifier(name) if matches!(name.text.as_str(), "print" | "println"))
    )
}
