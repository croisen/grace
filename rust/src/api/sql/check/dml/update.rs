use super::*;

pub(super) fn check(
    conditions: &mut [(bool, String)],
    diagnostics: &mut Diagnostics,
    expected: &SQLUpdate,
    actual: &SQLUpdate,
) {
    check_part(
        conditions,
        diagnostics,
        0,
        token_eq(&expected.table_name, &actual.table_name),
        "The UPDATE table does not match",
        Some(&actual.table_name),
    );
    for i in 0..expected.assignments.len().max(actual.assignments.len()) {
        let base = 1 + i * 2;
        let (Some(e), Some(a)) =
            (expected.assignments.get(i), actual.assignments.get(i))
        else {
            check_part(
                conditions,
                diagnostics,
                base,
                false,
                "An UPDATE assignment is missing or extra",
                actual.assignments.get(i).map(|x| &x.column),
            );
            check_part(
                conditions,
                diagnostics,
                base + 1,
                false,
                "An UPDATE value is missing or extra",
                actual.assignments.get(i).map(|x| &x.column),
            );
            continue;
        };
        check_part(
            conditions,
            diagnostics,
            base,
            token_eq(&e.column, &a.column),
            "The updated column does not match",
            Some(&a.column),
        );
        check_part(
            conditions,
            diagnostics,
            base + 1,
            expr_eq(&e.value, &a.value),
            "The assigned value does not match",
            expression_token(&a.value),
        );
    }
    let selection_index = 1 + expected.assignments.len() * 2;
    match (&expected.selection, &actual.selection) {
        (Some(e), Some(a)) => {
            let mut index = selection_index;
            check_predicate(
                conditions,
                diagnostics,
                &mut index,
                e,
                a,
                "UPDATE WHERE",
            );
        }
        (_, None) => missing_clause(
            diagnostics,
            conditions,
            selection_index,
            "The UPDATE WHERE condition is missing",
        ),
        (None, Some(a)) => check_part(
            conditions,
            diagnostics,
            selection_index,
            false,
            "Unexpected UPDATE WHERE condition",
            expression_token(a),
        ),
    }
}
