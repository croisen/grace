use oak_core::{Builder, ParseSession, SourceText};
use oak_sql::{SqlBuilder, SqlLanguage, SqlRoot};

use crate::api::diagnostics::Diagnostics;
use crate::api::grade::Grade;
use crate::api::sql::SqlConcept;

pub mod ddl;
pub mod dml;
pub mod query;

#[flutter_rust_bridge::frb(sync)]
pub fn sql_check(
    concept: SqlConcept,
    conditions: &mut Vec<(bool, String)>,
    generated: &str,
    user_code: &str,
) -> Diagnostics {
    let (uroot, mut d) = ast(user_code);
    let (groot, dd) = ast(generated);

    assert!(dd.is_empty());
    let generated = groot.unwrap();
    if let Some(user) = uroot {}

    d
}

fn ast(source: &str) -> (Option<SqlRoot>, Diagnostics) {
    let lang = SqlLanguage::new();
    let b = SqlBuilder::new(&lang);
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
