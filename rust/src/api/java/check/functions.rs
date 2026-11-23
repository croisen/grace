use std::mem::discriminant;

use oak_java::ast::{ClassDeclaration, Member, Statement};

use crate::api::diagnostics::Diagnostics;
use crate::api::java::check::utils::{
    check_print, check_variable_decl, compare_expr, generated_get_main,
    user_get_main,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn java_function_declaration(
    diagnostics: &mut Diagnostics,
    conditions: &mut Vec<(bool, String)>,
    generated: &ClassDeclaration,
    user: &ClassDeclaration,
) {
    let Some(umain) = user_get_main(diagnostics, user) else {
        return;
    };

    let gmain = generated_get_main(generated);

    let gfunc = &generated
        .members
        .iter()
        .filter_map(|a| {
            if let Member::Method(method) = a { Some(method) } else { None }
        })
        .find(|&m| m.name != "main")
        .unwrap();

    let ufunc = user
        .members
        .iter()
        .filter_map(|a| {
            if let Member::Method(method) = a { Some(method) } else { None }
        })
        .find(|&m| m.name != "main");

    if ufunc.is_none() {
        diagnostics.record_again("No other methods found than main");
        return;
    }

    let ufunc = ufunc.unwrap();
    if ufunc.body.is_empty() {
        diagnostics.record_again("Empty method");
        return;
    }

    conditions[0].0 = true;
    if gfunc.name != ufunc.name {
        diagnostics.record_hard(format!(
            "Expected method declaration name {} found {}",
            gfunc.name, ufunc.name
        ));

        conditions[0].0 = false;
    }

    let params = gfunc.parameters.iter().zip(ufunc.parameters.iter());
    for (ap, bp) in params {
        if ap.r#type != bp.r#type {
            diagnostics.record_hard(format!(
                "Expecting parameter type to be {} found {}",
                ap.r#type, bp.r#type
            ));

            conditions[0].0 = false;
        }

        if ap.name != bp.name {
            diagnostics.record_hard(format!(
                "Expecting parameter name to be {} found {}",
                ap.name, bp.name
            ));

            conditions[0].0 = false;
        }
    }

    conditions[1].0 = true;
    // well our drills make the functions return immediately (other than main of course)
    let body = gfunc.body.iter().zip(ufunc.body.iter());
    for (ab, bb) in body {
        if discriminant(ab) != discriminant(bb) {
            // change the message to be more specific (when all statement types has been turned into
            // strings
            diagnostics
                .record_hard("Found different statement type than expected");
            conditions[1].0 = false;
            break;
        }

        match (ab, bb) {
            (Statement::Return(None), Statement::Return(None)) => {
                conditions[2].0 = true
            }
            (Statement::Return(None), Statement::Return(_)) => {
                diagnostics.record_hard("Expecting return with no value");
                conditions[2].0 = false;
                break;
            }
            (Statement::Return(Some(a)), Statement::Return(Some(b))) => {
                conditions[2].0 = compare_expr(diagnostics, a, b);
            }
            _ => {}
        }
    }

    let mut idx = 0;
    if let Some(udecl) = umain.get(idx) {
        let gdecl = &gmain[0];
        idx += 1;
        conditions[3].0 = check_variable_decl(diagnostics, gdecl, udecl);
    }

    if let Some(uprint) = umain.get(idx) {
        let gprint = &gmain[1];
        conditions[4].0 = check_print(diagnostics, gprint, uprint);
    }
}
