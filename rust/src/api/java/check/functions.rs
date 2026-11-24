use super::*;

pub(super) fn check_function(
    d: &mut Diagnostics,
    c: &mut [(bool, String)],
    g: &JavaClass,
    u: &JavaClass,
) {
    let (Some(gf), Some(uf)) = (method(g, false), method(u, false)) else {
        missing(d, "No method declaration found besides main");
        return;
    };
    let mut signature = true;
    if gf.name.text != uf.name.text {
        mismatch_token(d, "The method name does not match the drill", &uf.name);
        signature = false;
    }
    if gf.return_type.text != uf.return_type.text {
        mismatch_token(
            d,
            "The method return type does not match the drill",
            &uf.return_type,
        );
        signature = false;
    }
    if gf.parameters.len() != uf.parameters.len() {
        mismatch(
            d,
            "The method parameter count does not match the drill",
            None,
        );
        signature = false;
    }
    for (a, b) in gf.parameters.iter().zip(&uf.parameters) {
        if a.name.text != b.name.text {
            mismatch_token(
                d,
                "A parameter name does not match the drill",
                &b.name,
            );
            signature = false;
        }
        if a.r#type.text != b.r#type.text {
            mismatch_token(
                d,
                "A parameter type does not match the drill",
                &b.r#type,
            );
            signature = false;
        }
        if a.is_varargs != b.is_varargs {
            mismatch_token(
                d,
                "A parameter's varargs setting does not match",
                &b.name,
            );
            signature = false;
        }
    }
    mark(c, 0, signature);
    if uf.body.is_empty() {
        mismatch(d, "The method body is empty", None);
        mark(c, 1, false);
        return;
    }
    mark(c, 1, true);
    let expected_result = gf.body.iter().find_map(return_value);
    let actual_result = uf.body.iter().find_map(return_value);
    let result = match (expected_result, actual_result) {
        (Some(a), Some(b)) => compare_expr(d, a, b),
        (_, None) => {
            mismatch(d, "The method must return a result", None);
            false
        }
        (None, _) => true,
    };
    mark(c, 2, result);
    let (Some(gmain), Some(umain)) = (main_body(g), main_body(u)) else {
        missing(d, "No main method found");
        return;
    };
    mark(c, 3, match (gmain.first(), umain.first()) {
        (Some(a), Some(b)) => check_variable(d, a, b),
        _ => {
            mismatch(d, "Expected a method call stored in a variable", None);
            false
        }
    });
    mark(c, 4, match (gmain.get(1), umain.get(1)) {
        (Some(a), Some(b)) => check_print(d, a, b),
        _ => {
            mismatch(d, "Expected a print statement", None);
            false
        }
    });
}
