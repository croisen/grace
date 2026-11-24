use super::*;

pub(super) fn check_switch(
    d: &mut Diagnostics,
    c: &mut [(bool, String)],
    g: &JavaClass,
    u: &JavaClass,
) {
    let (
        Some(JavaStatement::Switch { selector: gs, cases: gc, default: gd }),
        Some(JavaStatement::Switch { selector: us, cases: uc, default: ud }),
    ) = (
        find_statement(main_body(g).unwrap_or(&[]), is_switch),
        find_statement(main_body(u).unwrap_or(&[]), is_switch),
    )
    else {
        missing(d, "Expecting a switch statement");
        return;
    };
    mark(c, 0, compare_expr(d, gs, us));
    for (i, expected) in gc.iter().enumerate() {
        let index = 1 + i * 2;
        let Some(actual) = uc.get(i) else {
            mismatch(d, "A switch case is missing", None);
            mark(c, index, false);
            mark(c, index + 1, false);
            continue;
        };
        let label_ok = expected.label.text == actual.label.text;
        if !label_ok {
            mismatch_token(
                d,
                format!(
                    "Expected case `{}` but found `{}`",
                    expected.label.text, actual.label.text
                ),
                &actual.label,
            );
        }
        mark(c, index, label_ok);
        mark(c, index + 1, check_first_print(d, &expected.body, &actual.body));
    }
    let default_index = 1 + gc.len() * 2;
    if !gd.is_empty() && ud.is_empty() {
        mismatch(d, "The default case is missing", None);
    }
    mark(c, default_index, !ud.is_empty());
    mark(c, default_index + 1, check_first_print(d, gd, ud));
}
