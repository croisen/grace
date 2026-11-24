use super::*;

pub(super) fn check(
    conditions: &mut [(bool, String)],
    diagnostics: &mut Diagnostics,
    expected: &SQLDelete,
    actual: &SQLDelete,
) {
    check_part(
        conditions,
        diagnostics,
        0,
        token_eq(&expected.table_name, &actual.table_name),
        "The DELETE table does not match",
        Some(&actual.table_name),
    );
    match (&expected.selection, &actual.selection) {
        (Some(e), Some(a)) => {
            let mut index = 1;
            check_predicate(
                conditions,
                diagnostics,
                &mut index,
                e,
                a,
                "DELETE WHERE",
            );
        }
        (_, None) => missing_clause(
            diagnostics,
            conditions,
            1,
            "The DELETE WHERE condition is missing",
        ),
        (None, Some(a)) => check_part(
            conditions,
            diagnostics,
            1,
            false,
            "Unexpected DELETE WHERE condition",
            expression_token(a),
        ),
    }
}
