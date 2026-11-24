use oak_core::{Builder, ParseSession, SourceText};
use oak_rust::{RustBuilder, RustLanguage, RustRoot};

use crate::api::diagnostics::Diagnostics;
use crate::api::grade::Grade;
use crate::api::rust::RustConcept;

mod for_loops;
mod functions;
mod ifs;
mod loops;
mod match_statements;
mod utils;
mod variables;
mod while_loops;

#[flutter_rust_bridge::frb(sync)]
pub fn rust_check(
    concept: RustConcept,
    conditions: &mut Vec<(bool, String)>,
    generated: &str,
    user_code: &str,
) -> Diagnostics {
    let (uroot, mut d) = ast(user_code);
    let (groot, dd) = ast(generated);

    assert!(dd.is_empty());
    let generated = &groot.unwrap().items;
    if let Some(user) = uroot {}

    d
}

fn ast(source: &str) -> (Option<RustRoot>, Diagnostics) {
    let lang = RustLanguage::new();
    let b = RustBuilder::new(&lang);
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
