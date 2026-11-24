use super::*;

pub(super) fn check(
    conditions: &mut [(bool, String)],
    diagnostics: &mut Diagnostics,
    expected: &SQLInsert,
    actual: &SQLInsert,
) {
    check_part(
        conditions,
        diagnostics,
        0,
        token_eq(&expected.table_name, &actual.table_name),
        "The INSERT table does not match",
        Some(&actual.table_name),
    );
    for i in 0..expected.columns.len().max(actual.columns.len()) {
        let (Some(e), Some(a)) =
            (expected.columns.get(i), actual.columns.get(i))
        else {
            check_part(
                conditions,
                diagnostics,
                1 + i,
                false,
                "An INSERT column is missing or extra",
                actual.columns.get(i),
            );
            continue;
        };
        check_part(
            conditions,
            diagnostics,
            1 + i,
            token_eq(e, a),
            "The INSERT column does not match",
            Some(a),
        );
    }
    let value_base = 1 + expected.columns.len();
    for i in 0..expected.values.len().max(actual.values.len()) {
        let (Some(e), Some(a)) = (expected.values.get(i), actual.values.get(i))
        else {
            check_part(
                conditions,
                diagnostics,
                value_base + i,
                false,
                "An INSERT value is missing or extra",
                actual.values.get(i).and_then(expression_token),
            );
            continue;
        };
        let column = expected
            .columns
            .get(i)
            .map(|t| t.text.as_str())
            .unwrap_or("column");
        check_part(
            conditions,
            diagnostics,
            value_base + i,
            expr_eq(e, a),
            &format!("The value for {column} does not match"),
            expression_token(a),
        );
    }
}
