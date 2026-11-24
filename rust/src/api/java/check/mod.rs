use crate::api::diagnostics::Diagnostics;
use crate::api::java::JavaConcept;
use crate::core::parser::pratt::Token2;
use crate::langs::java::ast::JavaAst;
use crate::langs::java::ast::class::JavaClass;
use crate::langs::java::ast::expression::{JavaExpression, JavaMethodCall};
use crate::langs::java::ast::member::JavaMember;
use crate::langs::java::ast::method::JavaMethod;
use crate::langs::java::ast::statement::JavaStatement;

mod utils;
mod variables;
use utils::*;
mod do_while;
mod for_loops;
mod functions;
mod ifs;
mod switch_case;
mod while_loops;

#[flutter_rust_bridge::frb(ignore)]
pub fn java_check(
    concept: JavaConcept,
    conditions: &mut Vec<(bool, String)>,
    generated: &str,
    user_code: &str,
) -> Diagnostics {
    conditions.iter_mut().for_each(|(passed, _)| *passed = false);
    if user_code.trim().is_empty() {
        let mut diagnostics = Diagnostics::new(Vec::new());
        diagnostics.record_again("Empty answer", None, None);
        return diagnostics;
    }
    let generated = JavaAst::new(generated);
    let user = JavaAst::new(user_code);
    let mut diagnostics = Diagnostics::new(user.errors);
    if !generated.errors.is_empty() || generated.root.is_empty() {
        diagnostics.record_again(
            "The generated drill could not be parsed",
            None,
            None,
        );
        return diagnostics;
    }
    let Some(expected_class) = generated.root[0].classes.first() else {
        diagnostics.record_again(
            "The generated drill has no class",
            None,
            None,
        );
        return diagnostics;
    };
    let Some(actual_class) =
        user.root.first().and_then(|ast| ast.classes.first())
    else {
        diagnostics.record_again("No class found", None, None);
        return diagnostics;
    };

    match concept {
        JavaConcept::VarDecl => variables::check_variables(
            &mut diagnostics,
            conditions,
            expected_class,
            actual_class,
        ),
        JavaConcept::FunctionDecl => functions::check_function(
            &mut diagnostics,
            conditions,
            expected_class,
            actual_class,
        ),
        JavaConcept::Ifs => ifs::check_if(
            &mut diagnostics,
            conditions,
            expected_class,
            actual_class,
        ),
        JavaConcept::ForLoop => for_loops::check_for(
            &mut diagnostics,
            conditions,
            expected_class,
            actual_class,
        ),
        JavaConcept::WhileLoop => while_loops::check_while(
            &mut diagnostics,
            conditions,
            expected_class,
            actual_class,
        ),
        JavaConcept::DoWhile => do_while::check_do_while(
            &mut diagnostics,
            conditions,
            expected_class,
            actual_class,
        ),
        JavaConcept::Switch => switch_case::check_switch(
            &mut diagnostics,
            conditions,
            expected_class,
            actual_class,
        ),
    }
    diagnostics
}
