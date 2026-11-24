use super::*;

pub(super) fn check(
    conditions: &mut [(bool, String)],
    diagnostics: &mut Diagnostics,
    expected: &Token2,
    actual: &Token2,
) {
    check_part(
        conditions,
        diagnostics,
        0,
        true,
        "Use TRUNCATE TABLE",
        Some(actual),
    );
    check_part(
        conditions,
        diagnostics,
        1,
        token_eq(expected, actual),
        "The table name does not match",
        Some(actual),
    );
}
