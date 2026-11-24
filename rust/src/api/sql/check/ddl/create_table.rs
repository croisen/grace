use super::*;

pub(super) fn check(
    conditions: &mut [(bool, String)],
    diagnostics: &mut Diagnostics,
    expected: &crate::langs::sql::ast::statements::ddl::SQLCreate,
    actual: &crate::langs::sql::ast::statements::ddl::SQLCreate,
) {
    check_part(
        conditions,
        diagnostics,
        0,
        token_eq(&expected.name, &actual.name),
        "The table name does not match",
        Some(&actual.name),
    );
    let (SQLCreateBody::Table(ec), SQLCreateBody::Table(ac)) =
        (&expected.body, &actual.body)
    else {
        missing_clause(
            diagnostics,
            conditions,
            1,
            "Expected a CREATE TABLE column list",
        );
        return;
    };
    for i in 0..ec.len().max(ac.len()) {
        let (Some(e), Some(a)) = (ec.get(i), ac.get(i)) else {
            check_part(
                conditions,
                diagnostics,
                1 + i,
                false,
                "A table column is missing or extra",
                ac.get(i).map(|x| &x.name),
            );
            continue;
        };
        check_part(
            conditions,
            diagnostics,
            1 + i,
            column_definition_eq(e, a),
            "The column definition does not match",
            Some(&a.name),
        );
    }
}
