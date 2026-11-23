use oak_java::ast::ClassDeclaration;

use crate::api::diagnostics::Diagnostics;
use crate::api::java::check::utils::{
    check_print, check_variable_decl, generated_get_main, user_get_main,
};

#[flutter_rust_bridge::frb(ignore)]
pub fn java_variable_declaration(
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

    if let Some(uprint) = umain.get(idx) {
        let gprint = &gmain[1];
        conditions[1].0 = check_print(diagnostics, gprint, uprint);
    }
}
