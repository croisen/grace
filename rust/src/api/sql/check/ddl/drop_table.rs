use super::*;

pub(super) fn check(
    conditions: &mut [(bool, String)],
    diagnostics: &mut Diagnostics,
    expected: &crate::langs::sql::ast::statements::ddl::SQLDrop,
    actual: &crate::langs::sql::ast::statements::ddl::SQLDrop,
) {
    let kind_ok = matches!(
        &expected.r#type,
        crate::langs::sql::ast::statements::ddl::SQLDropType::Table
    ) && matches!(
        &actual.r#type,
        crate::langs::sql::ast::statements::ddl::SQLDropType::Table
    );
    check_part(
        conditions,
        diagnostics,
        0,
        kind_ok,
        "Use DROP TABLE",
        Some(&actual.name),
    );
    check_part(
        conditions,
        diagnostics,
        1,
        token_eq(&expected.name, &actual.name),
        "The table name does not match",
        Some(&actual.name),
    );
}
