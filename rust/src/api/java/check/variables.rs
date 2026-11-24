use super::*;

pub(super) fn check_variables(
    d: &mut Diagnostics,
    c: &mut [(bool, String)],
    g: &JavaClass,
    u: &JavaClass,
) {
    let (Some(gbody), Some(ubody)) = (main_body(g), main_body(u)) else {
        missing(d, "No main method found");
        return;
    };
    let (
        Some(JavaStatement::LocalVariable { name: gn, r#type: gt, init: gi }),
        Some(JavaStatement::LocalVariable { name: un, r#type: ut, init: ui }),
    ) = (gbody.first(), ubody.first())
    else {
        missing(d, "Expecting a variable declaration");
        return;
    };
    let mut ok = true;
    if gn.text != un.text {
        mismatch_token(
            d,
            format!("Expected variable `{}` but found `{}`", gn.text, un.text),
            un,
        );
        ok = false;
    }
    if gt.text != ut.text {
        mismatch_token(
            d,
            format!("Expected type `{}` but found `{}`", gt.text, ut.text),
            ut,
        );
        ok = false;
    }
    if !compare_optional_expr(d, gi.as_ref(), ui.as_ref()) {
        ok = false;
    }
    mark(c, 0, ok);
    let print_ok = match (gbody.get(1), ubody.get(1)) {
        (Some(gs), Some(us)) => check_print(d, gs, us),
        _ => {
            mismatch(d, "Expected a print statement", None);
            false
        }
    };
    mark(c, 1, print_ok);
}
