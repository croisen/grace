use oak_core::{Builder, ParseSession, SourceText};
use oak_rust::{RustBuilder, RustLanguage, RustRoot};

use crate::api::diagnostics::Diagnostics;

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
