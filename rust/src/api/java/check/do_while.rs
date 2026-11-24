use oak_java::ast::{ClassDeclaration, Statement};

use crate::api::diagnostics::Diagnostics;
use crate::api::java::check::utils::{
    check_print, check_variable_decl, compare_expr, generated_get_main,
    user_get_main,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn java_do_while(
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
        idx += 1;
        let gdecl = &gmain[0];
        conditions[0].0 = check_variable_decl(diagnostics, gdecl, udecl);
    }

    if let Some(uloop) = umain.get(idx) {
        let gloop = &gmain[1];

        let Statement::DoWhile { condition, body } = uloop else {
            diagnostics.record_again("Expecting do while loop");
            return;
        };

        conditions[1].0 = true;
        conditions[2].0 = true;
        let (ucond, ubody) = (condition, body);

        let Statement::DoWhile { condition, body } = gloop else {
            return;
        };

        let (gcond, gbody) = (condition, body);
        conditions[3].0 = compare_expr(diagnostics, gcond, ucond);

        let Statement::Block(ubody) = &**ubody else {
            diagnostics.record_hard("Expecting block in do while statement");
            return;
        };

        let Statement::Block(gbody) = &**gbody else {
            return;
        };

        if ubody.is_empty() {
            diagnostics.record_again("Empty do while statement");
            return;
        }

        conditions[4].0 = check_print(diagnostics, &gbody[0], &ubody[0]);
        if let Some(Statement::Expression(uupdate)) = ubody.get(1) {
            let Statement::Expression(gupdate) = &gbody[1] else {
                return;
            };

            conditions[5].0 = compare_expr(diagnostics, gupdate, uupdate);
        } else {
            diagnostics
                .record_hard("Missing update statement in do while loop");
        }
    }
}
