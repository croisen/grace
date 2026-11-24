use super::*;

pub(super) fn check_for(
    d: &mut Diagnostics,
    c: &mut [(bool, String)],
    g: &JavaClass,
    u: &JavaClass,
) {
    let (Some(expected), Some(actual)) = (
        find_statement(main_body(g).unwrap_or(&[]), is_for),
        find_statement(main_body(u).unwrap_or(&[]), is_for),
    ) else {
        missing(d, "Expecting a for loop");
        return;
    };
    let (
        JavaStatement::For { init: gi, condition: gc, update: gu, body: gb },
        JavaStatement::For { init: ui, condition: uc, update: uu, body: ub },
    ) = (expected, actual)
    else {
        mismatch(d, "The for loop form does not match the drill", None);
        mark(c, 0, false);
        return;
    };
    let init = match (gi.as_deref(), ui.as_deref()) {
        (Some(a), Some(b)) => check_variable(d, a, b),
        _ => {
            mismatch(d, "The for loop initializer is missing", None);
            false
        }
    };
    let cond = compare_optional_expr(d, gc.as_ref(), uc.as_ref());
    let update = compare_optional_expr(d, gu.as_ref(), uu.as_ref());
    mark(c, 0, init && cond && update);
    mark(c, 1, !statement_body(ub).is_empty());
    mark(c, 2, check_first_print(d, statement_body(gb), statement_body(ub)));
}
