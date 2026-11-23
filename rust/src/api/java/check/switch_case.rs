use oak_java::ast::{ClassDeclaration, Statement};

use crate::api::diagnostics::Diagnostics;
use crate::api::java::check::utils::{
    check_print, check_variable_decl, compare_expr, generated_get_main,
    user_get_main,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn java_switch_statement(
    diagnostics: &mut Diagnostics,
    conditions: &mut Vec<(bool, String)>,
    generated: &ClassDeclaration,
    user: &ClassDeclaration,
) {
    let Some(umain) = user_get_main(diagnostics, user) else {
        return;
    };

    let gmain = generated_get_main(generated);

    let mut idx = 0;
    if let Some(udecl) = umain.get(idx) {
        let gdecl = &gmain[0];
        idx += 1;
        let res = check_variable_decl(diagnostics, gdecl, udecl);
        conditions[0].0 = res;
        if !res {
            idx -= 1;
        }
    }

    if let Some(uswitch) = umain.get(idx) {
        let Statement::Switch { selector: usec, cases: ucase, default: udef } =
            uswitch
        else {
            diagnostics.record_again("Expecting switch statement");
            return;
        };

        let Statement::Switch {
            selector: gsec,
            cases: gcase,
            default: Some(gdef),
        } = &gmain[1]
        else {
            return;
        };

        conditions[1].0 = compare_expr(diagnostics, gsec, usec);
        if let Some(udef) = udef {
            conditions[8].0 = true;
            if let Some(uprint) = udef.get(0) {
                conditions[9].0 = check_print(diagnostics, &gdef[0], uprint);
            } else {
                diagnostics.record_good("Empty default case");
            }
        } else {
            diagnostics.record_good("Switch case is missing the default case");
        }

        let case = gcase.iter().zip(ucase.iter());
        let mut idx = 2;
        for (ac, bc) in case {
            conditions[idx].0 = compare_expr(diagnostics, &ac.label, &bc.label);
            if let Some(uprint) = bc.body.get(0) {
                conditions[idx + 1].0 =
                    check_print(diagnostics, &ac.body[0], uprint);
            } else {
                diagnostics.record_hard("Empty case statement");
            }

            idx += 2;
        }
    }
}
