use std::mem::discriminant;

use oak_java::ast::{ClassDeclaration, Expression, Member, Statement};

use crate::api::diagnostics::Diagnostics;
#[flutter_rust_bridge::frb(ignore)]
pub fn generated_get_main(g: &ClassDeclaration) -> &Vec<Statement> {
    &g.members
        .iter()
        .filter_map(|a| {
            if let Member::Method(method) = a { Some(method) } else { None }
        })
        .find(|&m| m.name == "main")
        .unwrap()
        .body
}

#[flutter_rust_bridge::frb(ignore)]
pub fn user_get_main<'a>(
    diagnostics: &mut Diagnostics,
    user: &'a ClassDeclaration,
) -> Option<&'a Vec<Statement>> {
    let main = user
        .members
        .iter()
        .filter_map(|a| {
            if let Member::Method(method) = a { Some(method) } else { None }
        })
        .find(|&m| m.name == "main");

    if main.is_none() {
        diagnostics.record_again("No main function");
        return None;
    }

    let body = &main.unwrap().body;
    if body.is_empty() {
        diagnostics.record_again("Empty main function");
        return None;
    }

    Some(&body)
}

#[flutter_rust_bridge::frb(ignore)]
pub fn compare_expr(
    diagnostics: &mut Diagnostics,
    generated: &Expression,
    user: &Expression,
) -> bool {
    let (g, u) = (expr_kind(generated), expr_kind(user));
    if discriminant(generated) != discriminant(user) {
        diagnostics.record_hard(format!("Expected expression {g} found {u}"));

        return false;
    }

    match (generated, user) {
        (Expression::Literal(a), Expression::Literal(b)) if a == b => true,
        (Expression::Literal(a), Expression::Literal(b)) => {
            diagnostics
                .record_hard(format!("Expected literal {a:?} found {b:?}",));

            false
        }
        (Expression::Identifier(a), Expression::Identifier(b)) if a == b => {
            true
        }
        (
            Expression::Unary { op: aop, expression: aexpr },
            Expression::Unary { op: bop, expression: bexpr },
        ) => {
            if aop != bop {
                diagnostics.record_hard(format!(
                    "Expecting unary op {aop} found {bop}"
                ));
                return false;
            }

            compare_expr(diagnostics, &*aexpr, &*bexpr)
        }
        (
            Expression::Binary { left: aleft, op: aop, right: aright },
            Expression::Binary { left: bleft, op: bop, right: bright },
        ) => {
            if aop != bop {
                if !aop.contains(['>', '<']) {
                    diagnostics.record_hard(format!(
                        "Expecting binary op {aop} found {bop}"
                    ));

                    return false;
                }

                diagnostics.record_good(format!(
                    "Expecting binary op {aop} found {bop}"
                ));
            }

            // it makes it so that it's bi-directional buut
            (compare_expr(diagnostics, aleft, bleft)
                || compare_expr(diagnostics, aleft, bright))
                && (compare_expr(diagnostics, aright, bright)
                    || compare_expr(diagnostics, aright, bleft))
        }
        (
            Expression::Update { expression: aexpr, op: aop, .. },
            Expression::Update { expression: bexpr, op: bop, .. },
        ) => {
            if aop != bop {
                diagnostics.record_hard(format!(
                    "Expecting update operator to be {aop} found {bop}"
                ));
                return false;
            }

            compare_expr(diagnostics, aexpr, bexpr)
        }
        _ => false,
    }
}

#[flutter_rust_bridge::frb(ignore)]
pub fn check_print(
    diagnostics: &mut Diagnostics,
    generated: &Statement,
    user: &Statement,
) -> bool {
    let Statement::Expression(Expression::MethodCall(gfield)) = generated
    else {
        return false;
    };

    let Statement::Expression(Expression::MethodCall(ufield)) = user else {
        diagnostics.record_hard("Expected method call for print");
        return false;
    };

    if ufield.name != "print" && ufield.name != "println" {
        diagnostics.record_hard(format!(
            "Expecting method call name to be print or println found {}",
            ufield.name
        ));
        return false;
    }

    let args = gfield.arguments.iter().zip(ufield.arguments.iter());
    for arg in args {
        if !compare_expr(diagnostics, arg.0, arg.1) {
            return false;
        }
    }

    true
}

#[flutter_rust_bridge::frb(ignore)]
pub fn check_variable_decl(
    diagnostics: &mut Diagnostics,
    generated: &Statement,
    user: &Statement,
) -> bool {
    let Statement::LocalVariable { r#type, name, initializer } = user else {
        diagnostics.record_again("Expecting variable declaration");
        return false;
    };

    let (utype, uname, uinit) = (r#type, name, initializer);
    let Statement::LocalVariable { r#type, name, initializer } = generated
    else {
        diagnostics.record_again("Expecting variable declaration");
        return false;
    };

    let (gtype, gname, ginit) = (r#type, name, initializer);

    if gtype != utype {
        diagnostics
            .record_again(format!("Expecting type to be {gtype} not {utype}"));
        return false;
    }

    if gname != uname {
        diagnostics.record_hard(format!(
            "Expecting variable name to be {gname} not {uname}"
        ));

        return false;
    }

    if ginit.is_some() && uinit.is_none() {
        diagnostics.record_hard("Expecting variable initialization");
        false
    } else if let (Some(ginit), Some(uinit)) = (ginit, uinit) {
        compare_expr(diagnostics, ginit, uinit)
    } else {
        diagnostics
            .record_good("Unexpected initializer for variable declaration");
        true
    }
}

fn expr_kind(expr: &Expression) -> &str {
    match expr {
        Expression::Literal(_) => "literal",
        Expression::Identifier(_) => "identifier",
        Expression::MethodCall(_) => "method call",
        Expression::FieldAccess(_) => "field access",
        Expression::ArrayAccess(_) => "array access",
        Expression::ArrayCreation(_) => "array creation",
        Expression::New(_) => "new",
        Expression::AnonymousClass(_) => "anonymous class",
        Expression::This => "this",
        Expression::Super => "super",
        Expression::Binary { .. } => "binary",
        Expression::Unary { .. } => "unary",
        Expression::Assignment { .. } => "assignment",
        Expression::Update { .. } => "update",
        Expression::Ternary { .. } => "ternary",
        Expression::Cast { .. } => "cast",
        Expression::Lambda(_) => "lambda",
    }
}
