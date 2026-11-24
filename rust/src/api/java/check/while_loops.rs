use super::*;

pub(super) fn check_while(
    d: &mut Diagnostics,
    c: &mut [(bool, String)],
    g: &JavaClass,
    u: &JavaClass,
) {
    let (Some(gmain), Some(umain)) = (main_body(g), main_body(u)) else {
        missing(d, "No main method found");
        return;
    };
    let variable = match (gmain.first(), umain.first()) {
        (Some(expected), Some(actual)) => check_variable(d, expected, actual),
        _ => {
            mismatch(d, "Expected a loop variable declaration", None);
            false
        }
    };
    mark(c, 0, variable);
    let (
        Some(JavaStatement::While { condition: gc, body: gb }),
        Some(JavaStatement::While { condition: uc, body: ub }),
    ) = (find_statement(gmain, is_while), find_statement(umain, is_while))
    else {
        missing(d, "Expecting a while loop");
        return;
    };
    mark(c, 1, compare_expr(d, gc, uc));
    mark(c, 2, !statement_body(ub).is_empty());
    mark(c, 3, check_first_print(d, statement_body(gb), statement_body(ub)));
    mark(c, 4, check_loop_update(d, statement_body(gb), statement_body(ub)));
}
