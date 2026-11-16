use oak_core::{Builder, ParseSession, SourceText};
use oak_java::{JavaBuilder, JavaLanguage};

use crate::api::diagnostics::Diagnostics;

pub fn java_to_ast(source: String) {
    let lang = JavaLanguage::new();
    let b = JavaBuilder::new(&lang);
    let mut c = ParseSession::default();

    let s = SourceText::new(source.as_str());
    let mut a = b.build(&s, &[], &mut c);

    match a.result {
        Ok(_) => println!("No fatal errors"),
        Err(e) => a.diagnostics.push(e),
    }

    let d = Diagnostics::new(&source, a.diagnostics);
    for err in d {
        println!("{}", err.to_string());
    }
}
