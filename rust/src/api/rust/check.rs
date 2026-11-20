use oak_core::{Builder, ParseSession, SourceText};
use oak_rust::{RustBuilder, RustLanguage};

use crate::api::diagnostics::Diagnostics;

pub fn rust_to_ast(source: String) {
    let lang = RustLanguage::new();
    let b = RustBuilder::new(&lang);
    let mut c = ParseSession::default();

    let s = SourceText::new(source.as_str());
    let mut a = b.build(&s, &[], &mut c);

    match a.result {
        Ok(root) => println!("{root:#?}\n"),
        Err(e) => a.diagnostics.push(e),
    }

    let d = Diagnostics::new(&source, a.diagnostics);
    for err in d {
        println!("{}", err.to_string());
    }
}
