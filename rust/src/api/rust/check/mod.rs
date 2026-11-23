use oak_core::{Builder, ParseSession, SourceText};
use oak_rust::{RustBuilder, RustLanguage, RustRoot};

use crate::api::diagnostics::Diagnostics;
use crate::api::grade::Grade;
use crate::api::rust::RustConcept;

pub mod for_loops;
pub mod functions;
pub mod ifs;
pub mod loops;
pub mod match_statements;
pub mod variables;
pub mod while_loops;

#[flutter_rust_bridge::frb(sync)]
pub fn rust_check(
    concept: RustConcept,
    conditions: Vec<(bool, String)>,
    generated: String,
    user: String,
) -> (Grade, Diagnostics) {
    let (uroot, mut d) = ast(user);
    let (groot, dd) = ast(generated);

    assert!(dd.is_empty());
    let generated = groot.unwrap();
    if let Some(user) = uroot {
        println!("{generated:#?}");

        println!("{user:#?}");
        (Grade::Good, d)
    } else {
        (d.iter().map(|d| d.grade).min().unwrap(), d)
    }
}

fn ast(source: String) -> (Option<RustRoot>, Diagnostics) {
    let lang = RustLanguage::new();
    let b = RustBuilder::new(&lang);
    let mut c = ParseSession::default();

    let mut r = None;
    let s = SourceText::new(source.as_str());
    let mut a = b.build(&s, &[], &mut c);

    match a.result {
        Ok(root) => r = Some(root),
        Err(e) => a.diagnostics.push(e),
    }

    (r, Diagnostics::new(&source, a.diagnostics))
}
