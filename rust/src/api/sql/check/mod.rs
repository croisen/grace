use oak_core::{Builder, ParseSession, SourceText};
use oak_sql::{SqlBuilder, SqlLanguage, SqlRoot};

use crate::api::diagnostics::Diagnostics;
use crate::api::grade::Grade;
use crate::api::sql::SqlConcept;

pub mod ddl;
pub mod dml;
pub mod query;

pub fn sql_check(
    concept: SqlConcept,
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

fn ast(source: String) -> (Option<SqlRoot>, Diagnostics) {
    let lang = SqlLanguage::new();
    let b = SqlBuilder::new(&lang);
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
