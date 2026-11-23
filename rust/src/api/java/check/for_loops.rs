use oak_java::ast::{ClassDeclaration, Statement};

use crate::api::diagnostics::Diagnostics;
use crate::api::java::check::utils::{
    check_print, check_variable_decl, compare_expr, generated_get_main,
    user_get_main,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn java_for_loops(
    diagnostics: &mut Diagnostics,
    conditions: &mut Vec<(bool, String)>,
    generated: &ClassDeclaration,
    user: &ClassDeclaration,
) {
    let Some(umain) = user_get_main(diagnostics, user) else {
        return;
    };

    let gmain = generated_get_main(generated);

    if let Some(uloop) = umain.get(0) {
        let gloop = &gmain[0];

        let Statement::For { init, condition, update, body } = uloop else {
            diagnostics.record_again("Expecting for loop");
            return;
        };

        let (uinit, ucond, uupdate, ubody) = (init, condition, update, body);

        let Statement::For { init, condition, update, body } = gloop else {
            return;
        };

        let (ginit, gcond, gupdate, gbody) = (
            init.as_ref().unwrap(),
            condition.as_ref().unwrap(),
            update.as_ref().unwrap(),
            body,
        );

        let mut for_loop = true;
        if let Some(uinit) = uinit.as_ref() {
            if let Statement::LocalVariable { .. } = &**uinit {
                for_loop = check_variable_decl(diagnostics, ginit, uinit);
            } else if let Statement::Expression(uinit) = &**uinit {
                diagnostics.record_good("Expecting variable declaration + initialization in for loop");
                let Statement::LocalVariable { initializer, .. } = &**ginit
                else {
                    return;
                };

                let ginit = initializer.as_ref().unwrap();
                for_loop = compare_expr(diagnostics, ginit, uinit)
            }
        } else {
            diagnostics.record_hard("No for loop init found");
            for_loop = false;
        }

        if let Some(ucond) = ucond.as_ref() {
            if for_loop {
                for_loop = compare_expr(diagnostics, gcond, ucond);
            }
        } else {
            diagnostics.record_hard("For loop condition is missing");
            if for_loop {
                for_loop = false;
            }
        }

        if let Some(uupdate) = uupdate.as_ref() {
            if for_loop {
                for_loop = compare_expr(diagnostics, gupdate, uupdate);
            }
        } else {
            diagnostics.record_hard("For loop update is missing");
            if for_loop {
                for_loop = false;
            }
        }

        conditions[0].0 = for_loop;
        let Statement::Block(gbody) = &**gbody else {
            return;
        };

        if let Statement::Expression(_) = &**ubody {
            conditions[1].0 = true;
            conditions[2].0 = check_print(diagnostics, &gbody[0], ubody);
        } else if let Statement::Block(ubody) = &**ubody {
            if ubody.is_empty() {
                diagnostics.record_hard("Empty for loop body");
                return;
            }

            conditions[1].0 = true;
            conditions[2].0 = check_print(diagnostics, &gbody[0], &ubody[0]);
        } else {
            conditions[1].0 = false;
        }
    }
}
