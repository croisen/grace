use std::fmt::Debug;

use oak_core::{Builder, ParseSession, SourceText};
use oak_sql::{SqlBuilder, SqlLanguage, SqlRoot};

use crate::api::diagnostics::Diagnostics;
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
    for (passed, _) in conditions.iter_mut() {
        *passed = false;
    }

    if user_code.trim().is_empty() {
        let (_, mut diagnostics) = ast(user_code);
        diagnostics.record_again("Empty answer");
        return diagnostics;
    }

    // Oak's current SQL AST does not model TRUNCATE or every ALTER action used
    // by the drill generator. Grade those complete statements textually.
    if matches!(concept, SqlConcept::TruncateTable | SqlConcept::AlterTable) {
        if normalize_sql(generated) == normalize_sql(user_code) {
            for (passed, _) in conditions.iter_mut() {
                *passed = true;
            }
        } else {
            let (_, mut diagnostics) = ast(user_code);
            diagnostics.record_hard(mismatch_message(concept));
            return diagnostics;
        }
        return Diagnostics::new(user_code, Vec::new());
    }

    let (uroot, mut diagnostics) = ast(user_code);
    let Some(user) = uroot else {
        return diagnostics;
    };

    let (groot, generated_diagnostics) = ast(generated);
    let Some(generated_root) = groot else {
        diagnostics
            .record_again("The generated drill answer could not be parsed");
        return diagnostics;
    };
    if !generated_diagnostics.d.is_empty() {
        diagnostics
            .record_again("The generated drill answer contains a syntax error");
        return diagnostics;
    }

    if user.statements.is_empty() {
        diagnostics.record_again("No SQL statement found");
        return diagnostics;
    }
    if user.statements.len() != generated_root.statements.len() {
        diagnostics.record_hard(mismatch_message(concept));
        return diagnostics;
    }

    let matches = check_concept(
        concept,
        generated_root.statements.first(),
        user.statements.first(),
        generated,
        user_code,
    );
    if matches {
        for (passed, _) in conditions.iter_mut() {
            *passed = true;
        }
    } else {
        diagnostics.record_hard(mismatch_message(concept));
    }

    diagnostics
}

fn check_concept(
    concept: SqlConcept,
    generated: Option<&oak_sql::ast::SqlStatement>,
    user: Option<&oak_sql::ast::SqlStatement>,
    generated_source: &str,
    user_source: &str,
) -> bool {
    use oak_sql::ast::SqlStatement as S;

    let (Some(generated), Some(user)) = (generated, user) else {
        return false;
    };

    match concept {
        SqlConcept::Select => match (generated, user) {
            (S::Select(g), S::Select(u)) => {
                ast_eq(&g.items, &u.items) && ast_eq(&g.from, &u.from)
            }
            _ => false,
        },
        SqlConcept::Where => match (generated, user) {
            (S::Select(g), S::Select(u)) => {
                query_base_matches(g, u) && ast_eq(&g.expr, &u.expr)
            }
            _ => false,
        },
        SqlConcept::OrderBy => match (generated, user) {
            (S::Select(g), S::Select(u)) => {
                query_base_matches(g, u) && ast_eq(&g.order_by, &u.order_by)
            }
            _ => false,
        },
        SqlConcept::GroupBy => match (generated, user) {
            (S::Select(g), S::Select(u)) => {
                query_base_matches(g, u) && ast_eq(&g.group_by, &u.group_by)
            }
            _ => false,
        },
        SqlConcept::Having => match (generated, user) {
            (S::Select(g), S::Select(u)) => {
                query_base_matches(g, u)
                    && ast_eq(&g.group_by, &u.group_by)
                    && ast_eq(&g.having, &u.having)
            }
            _ => false,
        },
        SqlConcept::Join => match (generated, user) {
            (S::Select(g), S::Select(u)) => {
                query_base_matches(g, u) && ast_eq(&g.joins, &u.joins)
            }
            _ => false,
        },
        SqlConcept::Aggregate => match (generated, user) {
            (S::Select(g), S::Select(u)) => {
                query_base_matches(g, u) && ast_eq(&g.items, &u.items)
            }
            _ => false,
        },
        // DML and DDL drills each check one AST. Source spans are removed from
        // the debug representation before comparing the tree shapes and values.
        SqlConcept::Insert => ast_eq(generated, user),
        SqlConcept::Update => ast_eq(generated, user),
        SqlConcept::Delete => ast_eq(generated, user),
        SqlConcept::CreateTable => ast_eq(generated, user),
        SqlConcept::AlterTable => ast_eq(generated, user),
        SqlConcept::DropTable => ast_eq(generated, user),
        SqlConcept::TruncateTable => {
            normalize_sql(generated_source) == normalize_sql(user_source)
        }
        SqlConcept::AddColumn
        | SqlConcept::DropColumn
        | SqlConcept::RenameTable => {
            matches!((generated, user), (S::Alter(_), S::Alter(_)))
                && ast_eq(generated, user)
        }
    }
}

fn query_base_matches(
    generated: &oak_sql::ast::SelectStatement,
    user: &oak_sql::ast::SelectStatement,
) -> bool {
    ast_eq(&generated.items, &user.items) && ast_eq(&generated.from, &user.from)
}

fn ast_eq<T: Debug>(generated: &T, user: &T) -> bool {
    without_spans(&format!("{generated:?}"))
        == without_spans(&format!("{user:?}"))
}

fn without_spans(debug: &str) -> String {
    let mut result = String::with_capacity(debug.len());
    let mut rest = debug;
    while let Some(index) = rest.find("span: ") {
        result.push_str(&rest[..index]);
        let after = &rest[index + "span: ".len()..];
        let end =
            after.find(|ch| ch == ',' || ch == '}').unwrap_or(after.len());
        rest = &after[end..];
    }
    result.push_str(rest);
    result
}

fn mismatch_message(concept: SqlConcept) -> &'static str {
    match concept {
        SqlConcept::Select => {
            "The selected columns or table do not match the expected answer"
        }
        SqlConcept::Where => {
            "The WHERE condition does not match the expected answer"
        }
        SqlConcept::OrderBy => {
            "The ORDER BY clause does not match the expected answer"
        }
        SqlConcept::GroupBy => {
            "The GROUP BY clause does not match the expected answer"
        }
        SqlConcept::Having => {
            "The HAVING condition does not match the expected answer"
        }
        SqlConcept::Join => {
            "The joined table or join condition does not match the expected answer"
        }
        SqlConcept::Aggregate => {
            "The aggregate expression does not match the expected answer"
        }
        SqlConcept::Insert => {
            "The INSERT operation does not match the expected answer"
        }
        SqlConcept::Update => {
            "The UPDATE operation does not match the expected answer"
        }
        SqlConcept::Delete => {
            "The DELETE operation does not match the expected answer"
        }
        SqlConcept::CreateTable => {
            "The table definition does not match the expected answer"
        }
        SqlConcept::AlterTable => {
            "The ALTER TABLE operation does not match the expected answer"
        }
        SqlConcept::DropTable => {
            "The DROP TABLE operation does not match the expected answer"
        }
        SqlConcept::TruncateTable => {
            "The TRUNCATE TABLE operation does not match the expected answer"
        }
        SqlConcept::AddColumn => {
            "The added column does not match the expected answer"
        }
        SqlConcept::DropColumn => {
            "The dropped column does not match the expected answer"
        }
        SqlConcept::RenameTable => {
            "The table rename does not match the expected answer"
        }
    }
}

fn normalize_sql(source: &str) -> String {
    let mut normalized = String::with_capacity(source.len());
    let mut quote: Option<char> = None;
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        if let Some(delimiter) = quote {
            normalized.push(ch);
            if ch == delimiter {
                if chars.peek() == Some(&delimiter) {
                    normalized.push(chars.next().unwrap());
                } else {
                    quote = None;
                }
            }
            continue;
        }

        match ch {
            '\'' | '"' | '`' => {
                quote = Some(ch);
                normalized.push(ch);
            }
            c if c.is_whitespace() => {}
            c => normalized.extend(c.to_uppercase()),
        }
    }
    normalized
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
