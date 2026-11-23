use oak_java::ast::{ClassDeclaration, Statement};

use crate::api::diagnostics::Diagnostics;
use crate::api::java::check::utils::{
    check_print, check_variable_decl, compare_expr, generated_get_main,
    user_get_main,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn java_if_statements(
    diagnostics: &mut Diagnostics,
    conditions: &mut Vec<(bool, String)>,
    generated: &ClassDeclaration,
    user: &ClassDeclaration,
) {
    let Some(umain) = user_get_main(diagnostics, user) else {
        return;
    };

    let gmain = generated_get_main(generated);

    println!("{user:#?}");
    if let Some(uif) = umain.get(0) {
        let gif = &gmain[0];
        let Statement::If {
            condition: ucond,
            then_branch: uthen,
            else_branch: uelse,
        } = uif
        else {
            diagnostics.record_again("Expecting if statement");
            return;
        };

        let Statement::If {
            condition: gcond,
            then_branch: gthen,
            else_branch: Some(gelse),
        } = gif
        else {
            return;
        };

        conditions[0].0 = compare_expr(diagnostics, gcond, ucond);

        let Statement::Block(gthen) = &**gthen else {
            return;
        };

        let gthen = &gthen[0];

        if let Statement::Expression(_) = &**uthen {
            let res = check_print(diagnostics, gthen, uthen);
            conditions[1].0 = res;
            conditions[2].0 = res;
        } else if let Statement::Block(uthen) = &**uthen {
            if let Some(uprint) = uthen.get(0) {
                let res = check_print(diagnostics, gthen, uprint);
                conditions[1].0 = res;
                conditions[2].0 = res;
            } else {
                diagnostics.record_hard("Empty if statement");
            }
        }

        if uelse.is_none() {
            diagnostics.record_again(
                "Expecting else statement for the previous if statement",
            );

            return;
        }

        let Statement::Block(gelse) = &**gelse else {
            return;
        };

        let gelse = &gelse[0];
        let uelse = uelse.as_ref().unwrap();

        if let Statement::Expression(_) = &**uelse {
            let res = check_print(diagnostics, gelse, uelse);
            conditions[3].0 = res;
            conditions[4].0 = res;
        } else if let Statement::Block(uelse) = &**uelse {
            if let Some(uprint) = uelse.get(0) {
                let res = check_print(diagnostics, gelse, uprint);
                conditions[3].0 = res;
                conditions[4].0 = res;
            } else {
                diagnostics.record_hard("Empty else statement");
            }
        }
    }
}
