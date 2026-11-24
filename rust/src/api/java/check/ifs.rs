use super::*;

pub(super) fn check_if(
    d: &mut Diagnostics,
    c: &mut [(bool, String)],
    g: &JavaClass,
    u: &JavaClass,
) {
    let (Some(gif), Some(uif)) = (
        find_statement(main_body(g).unwrap_or(&[]), is_if),
        find_statement(main_body(u).unwrap_or(&[]), is_if),
    ) else {
        missing(d, "Expecting an if statement");
        return;
    };
    let (
        JavaStatement::If { condition: gc, then: gt, r#else: ge },
        JavaStatement::If { condition: uc, then: ut, r#else: ue },
    ) = (gif, uif)
    else {
        return;
    };
    mark(c, 0, compare_expr(d, gc, uc));
    mark(c, 1, !statement_body(ut).is_empty());
    mark(c, 2, check_first_print(d, statement_body(gt), statement_body(ut)));
    if ge.is_none() || ue.is_none() {
        mismatch(d, "Expected an else block", None);
        mark(c, 3, false);
        mark(c, 4, false);
        return;
    }
    mark(c, 3, true);
    mark(
        c,
        4,
        check_first_print(
            d,
            statement_body(ge.as_deref().unwrap()),
            statement_body(ue.as_deref().unwrap()),
        ),
    );
}
