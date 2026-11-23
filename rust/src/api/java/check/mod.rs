use oak_core::{Builder, ParseSession, SourceText};
use oak_java::ast::Item;
use oak_java::{JavaBuilder, JavaLanguage, JavaRoot};

use crate::api::diagnostics::Diagnostics;
use crate::api::java::JavaConcept;
use crate::api::java::check::do_while::java_do_while;
use crate::api::java::check::for_loops::java_for_loops;

mod do_while;
mod for_loops;
mod functions;
mod ifs;
mod switch_case;
mod utils;
mod variables;
mod while_loops;

#[flutter_rust_bridge::frb(ignore)]
pub fn java_check(
    concept: JavaConcept,
    conditions: &mut Vec<(bool, String)>,
    generated: &str,
    user_code: &str,
) -> Diagnostics {
    let (uroot, mut d) = ast(user_code);
    let (groot, dd) = ast(generated);

    assert!(dd.is_empty());
    let groot = groot.unwrap();
    let Item::Class(generated) =
        groot.items.iter().find(|&x| matches!(x, Item::Class(_))).unwrap()
    else {
        unreachable!()
    };

    if let Some(user) = uroot {
        if user.items.is_empty() {
            d.record_again("Empty answer");
        } else {
            let class =
                user.items.iter().find(|&x| matches!(x, Item::Class(_)));
            if let Some(Item::Class(class)) = class {
                match concept {
                    JavaConcept::DoWhile => {
                        java_do_while(&mut d, conditions, generated, class);
                    }
                    JavaConcept::ForLoop => {
                        java_for_loops(&mut d, conditions, generated, class)
                    }
                    _ => {}
                }
            } else {
                d.record_again("No classes found");
            }
        }
    }

    d
}

fn ast(source: &str) -> (Option<JavaRoot>, Diagnostics) {
    let lang = JavaLanguage::new();
    let b = JavaBuilder::new(&lang);
    let mut c = ParseSession::default();

    let mut r = None;
    let s = SourceText::new(source);
    let mut a = b.build(&s, &[], &mut c);

    match a.result {
        Ok(root) => r = Some(root),
        Err(e) => a.diagnostics.push(e),
    }

    (r, Diagnostics::new(&source, a.diagnostics))
}
