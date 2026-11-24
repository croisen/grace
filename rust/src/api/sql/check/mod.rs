use crate::api::diagnostics::Diagnostics;
use crate::api::sql::SqlConcept;
use crate::core::edit::{close_enough, edit_distance_sen};
use crate::core::parser::pratt::Token2;
use crate::langs::sql::ast::SQLAst;
use crate::langs::sql::ast::expression::SQLExpression;
use crate::langs::sql::ast::statements::ddl::{
    SQLAlterAction, SQLColumnDefinition, SQLColumnRestraint, SQLCreateBody,
};
use crate::langs::sql::ast::statements::dml::{
    SQLDelete, SQLInsert, SQLUpdate,
};
use crate::langs::sql::ast::statements::query::{SQLSelect, SQLSelectItem};

#[path = "ddl/alter_table.rs"]
mod alter_table_checker;
#[path = "ddl/create_table.rs"]
mod create_table_checker;
#[path = "dml/delete.rs"]
mod delete_checker;
#[path = "ddl/drop_table.rs"]
mod drop_table_checker;
#[path = "dml/insert.rs"]
mod insert_checker;
#[path = "query/select.rs"]
mod query_checker;
#[path = "ddl/truncate_table.rs"]
mod truncate_table_checker;
#[path = "dml/update.rs"]
mod update_checker;

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
        let mut diagnostics = Diagnostics::new(Vec::new());
        diagnostics.record_again("Empty answer", None, None);
        return diagnostics;
    }

    let generated = SQLAst::new(generated);
    if !generated.errors.is_empty() || generated.root.is_empty() {
        let mut diagnostics = Diagnostics::new(generated.errors);
        diagnostics.record_again(
            "The generated SQL drill could not be parsed",
            None,
            None,
        );
        return diagnostics;
    }

    let user = SQLAst::new(user_code);
    let mut diagnostics = Diagnostics::new(user.errors);
    if !diagnostics.is_empty() {
        return diagnostics;
    }
    let (Some(expected), Some(actual)) =
        (generated.root.first(), user.root.first())
    else {
        diagnostics.record_again("No SQL statement found", None, None);
        return diagnostics;
    };

    if generated.root.len() != 1 || user.root.len() != 1 {
        diagnostics.record_hard("Expected one SQL statement", None, None);
        return diagnostics;
    }

    match concept {
        SqlConcept::Select
        | SqlConcept::Where
        | SqlConcept::OrderBy
        | SqlConcept::GroupBy
        | SqlConcept::Having
        | SqlConcept::Join
        | SqlConcept::Aggregate => {
            let (SQLAst::Select(expected), SQLAst::Select(actual)) =
                (expected, actual)
            else {
                return missing_concept(
                    &mut diagnostics,
                    "Expected a SELECT statement",
                );
            };
            query_checker::check(
                concept,
                conditions,
                &mut diagnostics,
                expected,
                actual,
            );
        }
        SqlConcept::Insert => {
            let (SQLAst::Insert(expected), SQLAst::Insert(actual)) =
                (expected, actual)
            else {
                return missing_concept(
                    &mut diagnostics,
                    "Expected an INSERT statement",
                );
            };
            insert_checker::check(
                conditions,
                &mut diagnostics,
                expected,
                actual,
            );
        }
        SqlConcept::Update => {
            let (SQLAst::Update(expected), SQLAst::Update(actual)) =
                (expected, actual)
            else {
                return missing_concept(
                    &mut diagnostics,
                    "Expected an UPDATE statement",
                );
            };
            update_checker::check(
                conditions,
                &mut diagnostics,
                expected,
                actual,
            );
        }
        SqlConcept::Delete => {
            let (SQLAst::Delete(expected), SQLAst::Delete(actual)) =
                (expected, actual)
            else {
                return missing_concept(
                    &mut diagnostics,
                    "Expected a DELETE statement",
                );
            };
            delete_checker::check(
                conditions,
                &mut diagnostics,
                expected,
                actual,
            );
        }
        SqlConcept::CreateTable => {
            let (SQLAst::Create(expected), SQLAst::Create(actual)) =
                (expected, actual)
            else {
                return missing_concept(
                    &mut diagnostics,
                    "Expected a CREATE TABLE statement",
                );
            };
            create_table_checker::check(
                conditions,
                &mut diagnostics,
                expected,
                actual,
            );
        }
        SqlConcept::AlterTable
        | SqlConcept::AddColumn
        | SqlConcept::DropColumn
        | SqlConcept::RenameTable => {
            let (SQLAst::Alter(expected), SQLAst::Alter(actual)) =
                (expected, actual)
            else {
                return missing_concept(
                    &mut diagnostics,
                    "Expected an ALTER TABLE statement",
                );
            };
            alter_table_checker::check(
                concept,
                conditions,
                &mut diagnostics,
                &expected.name,
                expected.action.as_ref(),
                &actual.name,
                actual.action.as_ref(),
            );
        }
        SqlConcept::DropTable => {
            let (SQLAst::Drop(expected), SQLAst::Drop(actual)) =
                (expected, actual)
            else {
                return missing_concept(
                    &mut diagnostics,
                    "Expected a DROP TABLE statement",
                );
            };
            drop_table_checker::check(
                conditions,
                &mut diagnostics,
                expected,
                actual,
            );
        }
        SqlConcept::TruncateTable => {
            let (SQLAst::Truncate(expected), SQLAst::Truncate(actual)) =
                (expected, actual)
            else {
                return missing_concept(
                    &mut diagnostics,
                    "Expected a TRUNCATE TABLE statement",
                );
            };
            truncate_table_checker::check(
                conditions,
                &mut diagnostics,
                expected,
                actual,
            );
        }
    }

    diagnostics
}

fn missing_concept(
    diagnostics: &mut Diagnostics,
    message: &str,
) -> Diagnostics {
    diagnostics.record_again(message, None, None);
    std::mem::replace(diagnostics, Diagnostics::new(Vec::new()))
}

fn check_predicate(
    conditions: &mut [(bool, String)],
    diagnostics: &mut Diagnostics,
    index: &mut usize,
    expected: &SQLExpression,
    actual: &SQLExpression,
    clause: &str,
) {
    let expected_parts = flatten_and(expected);
    let actual_parts = flatten_and(actual);
    for i in 0..expected_parts.len().max(actual_parts.len()) {
        let (Some(expected), Some(actual)) =
            (expected_parts.get(i), actual_parts.get(i))
        else {
            check_part(
                conditions,
                diagnostics,
                *index,
                false,
                &format!("A {clause} predicate is missing or extra"),
                actual_parts.get(i).and_then(|expr| expression_token(expr)),
            );
            *index += 1;
            check_part(
                conditions,
                diagnostics,
                *index,
                false,
                &format!("A {clause} predicate operator is missing or extra"),
                None,
            );
            *index += 1;
            check_part(
                conditions,
                diagnostics,
                *index,
                false,
                &format!("A {clause} predicate value is missing or extra"),
                None,
            );
            *index += 1;
            continue;
        };
        let (Some((el, eo, er)), Some((al, ao, ar))) =
            (binary_parts(expected), binary_parts(actual))
        else {
            check_part(
                conditions,
                diagnostics,
                *index,
                expr_eq(expected, actual),
                &format!("The {clause} expression does not match"),
                expression_token(actual),
            );
            *index += 1;
            check_part(
                conditions,
                diagnostics,
                *index,
                false,
                &format!("The {clause} operator is missing"),
                expression_token(actual),
            );
            *index += 1;
            check_part(
                conditions,
                diagnostics,
                *index,
                false,
                &format!("The {clause} comparison value is missing"),
                expression_token(actual),
            );
            *index += 1;
            continue;
        };
        check_part(
            conditions,
            diagnostics,
            *index,
            expr_eq(el, al),
            &format!("The {clause} left expression does not match"),
            expression_token(al),
        );
        *index += 1;
        check_part(
            conditions,
            diagnostics,
            *index,
            token_eq(eo, ao),
            &format!("The {clause} operator does not match"),
            Some(ao),
        );
        *index += 1;
        check_part(
            conditions,
            diagnostics,
            *index,
            expr_eq(er, ar),
            &format!("The {clause} right expression does not match"),
            expression_token(ar),
        );
        *index += 1;
    }
}

fn flatten_and(expression: &SQLExpression) -> Vec<&SQLExpression> {
    match expression {
        SQLExpression::BinaryOp { op, left, right }
            if op.text.eq_ignore_ascii_case("AND") =>
        {
            let mut parts = flatten_and(left);
            parts.extend(flatten_and(right));
            parts
        }
        _ => vec![expression],
    }
}

fn missing_clause(
    diagnostics: &mut Diagnostics,
    conditions: &mut [(bool, String)],
    index: usize,
    message: &str,
) {
    mark(conditions, index, false);
    diagnostics.record_again(message, None, None);
}

fn check_part(
    conditions: &mut [(bool, String)],
    diagnostics: &mut Diagnostics,
    index: usize,
    passed: bool,
    message: &str,
    token: Option<&Token2>,
) {
    mark(conditions, index, passed);
    if !passed {
        let (line, col) = token
            .map(|token| (Some(token.line), Some(token.col)))
            .unwrap_or((None, None));
        diagnostics.record_hard(message, line, col);
    }
}

fn mark(conditions: &mut [(bool, String)], index: usize, passed: bool) {
    if let Some((value, _)) = conditions.get_mut(index) {
        *value = passed;
    }
}

fn token_eq(a: &Token2, b: &Token2) -> bool {
    close_enough(&a.text, edit_distance_sen(&a.text, &b.text))
}

fn debug_eq<T: std::fmt::Debug>(a: &T, b: &T) -> bool {
    normalize_debug(&format!("{a:?}")) == normalize_debug(&format!("{b:?}"))
}

fn normalize_debug(input: &str) -> String {
    let mut result = input.to_string();
    for field in ["line: ", "col: "] {
        let mut from = 0;
        while let Some(found) = result[from..].find(field) {
            let start = from + found + field.len();
            let end = result[start..]
                .find(|ch: char| !ch.is_ascii_digit())
                .map(|i| start + i)
                .unwrap_or(result.len());
            result.replace_range(start..end, "_");
            from = start + 1;
        }
    }
    result
}

fn expr_eq(a: &SQLExpression, b: &SQLExpression) -> bool {
    match (a, b) {
        (SQLExpression::Literal(a), SQLExpression::Literal(b))
        | (SQLExpression::Identifier(a), SQLExpression::Identifier(b)) => {
            token_eq(a, b)
        }
        (
            SQLExpression::FunctionCall { name: an, args: aa },
            SQLExpression::FunctionCall { name: bn, args: ba },
        ) => {
            token_eq(an, bn)
                && aa.len() == ba.len()
                && aa.iter().zip(ba).all(|(a, b)| expr_eq(a, b))
        }
        (
            SQLExpression::UnaryOp { op: ao, expr: ae, is_prefix: ap },
            SQLExpression::UnaryOp { op: bo, expr: be, is_prefix: bp },
        ) => token_eq(ao, bo) && ap == bp && expr_eq(ae, be),
        (
            SQLExpression::BinaryOp { op: ao, left: al, right: ar },
            SQLExpression::BinaryOp { op: bo, left: bl, right: br },
        ) => token_eq(ao, bo) && expr_eq(al, bl) && expr_eq(ar, br),
        (SQLExpression::Vector(a), SQLExpression::Vector(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| expr_eq(a, b))
        }
        (
            SQLExpression::InList { expr: ae, list: al, negated: an },
            SQLExpression::InList { expr: be, list: bl, negated: bn },
        ) => {
            an == bn
                && expr_eq(ae, be)
                && al.len() == bl.len()
                && al.iter().zip(bl).all(|(a, b)| expr_eq(a, b))
        }
        (
            SQLExpression::Between { expr: ae, low: al, high: ah, negated: an },
            SQLExpression::Between { expr: be, low: bl, high: bh, negated: bn },
        ) => an == bn && expr_eq(ae, be) && expr_eq(al, bl) && expr_eq(ah, bh),
        (SQLExpression::SubQuery(a), SQLExpression::SubQuery(b)) => {
            debug_eq(a, b)
        }
        _ => false,
    }
}

fn select_item_eq(a: &SQLSelectItem, b: &SQLSelectItem) -> bool {
    match (a, b) {
        (SQLSelectItem::Asterisk, SQLSelectItem::Asterisk) => true,
        (SQLSelectItem::Expr(a, aa), SQLSelectItem::Expr(b, ba)) => {
            expr_eq(a, b)
                && match (aa, ba) {
                    (Some(a), Some(b)) => expr_eq(a, b),
                    (None, None) => true,
                    _ => false,
                }
        }
        _ => false,
    }
}

fn column_definition_eq(
    a: &SQLColumnDefinition,
    b: &SQLColumnDefinition,
) -> bool {
    token_eq(&a.name, &b.name)
        && token_eq(&a.r#type, &b.r#type)
        && a.constraints.len() == b.constraints.len()
        && a.constraints
            .iter()
            .zip(&b.constraints)
            .all(|(a, b)| column_restraint_eq(a, b))
}

fn column_restraint_eq(a: &SQLColumnRestraint, b: &SQLColumnRestraint) -> bool {
    match (a, b) {
        (SQLColumnRestraint::PrimaryKey, SQLColumnRestraint::PrimaryKey)
        | (SQLColumnRestraint::ForeignKey, SQLColumnRestraint::ForeignKey)
        | (SQLColumnRestraint::Unique, SQLColumnRestraint::Unique)
        | (SQLColumnRestraint::NotNull, SQLColumnRestraint::NotNull)
        | (SQLColumnRestraint::Nullable, SQLColumnRestraint::Nullable)
        | (
            SQLColumnRestraint::AutoIncrement,
            SQLColumnRestraint::AutoIncrement,
        ) => true,
        (SQLColumnRestraint::Default(a), SQLColumnRestraint::Default(b))
        | (SQLColumnRestraint::Check(a), SQLColumnRestraint::Check(b)) => {
            expr_eq(a, b)
        }
        _ => false,
    }
}

fn alter_action_eq(a: &SQLAlterAction, b: &SQLAlterAction) -> bool {
    match (a, b) {
        (
            SQLAlterAction::AddColumn(an, at),
            SQLAlterAction::AddColumn(bn, bt),
        ) => token_eq(an, bn) && token_eq(at, bt),
        (SQLAlterAction::DropColumn(a), SQLAlterAction::DropColumn(b))
        | (SQLAlterAction::RenameTable(a), SQLAlterAction::RenameTable(b)) => {
            token_eq(a, b)
        }
        (
            SQLAlterAction::RenameTo(an, at),
            SQLAlterAction::RenameTo(bn, bt),
        ) => token_eq(an, bn) && token_eq(at, bt),
        _ => false,
    }
}

fn binary_parts(
    expression: &SQLExpression,
) -> Option<(&SQLExpression, &Token2, &SQLExpression)> {
    if let SQLExpression::BinaryOp { op, left, right } = expression {
        Some((left, op, right))
    } else {
        None
    }
}

fn select_item_token(item: &SQLSelectItem) -> Option<&Token2> {
    match item {
        SQLSelectItem::Asterisk => None,
        SQLSelectItem::Expr(expr, _) => expression_token(expr),
    }
}

fn expression_token(expression: &SQLExpression) -> Option<&Token2> {
    match expression {
        SQLExpression::Identifier(token) | SQLExpression::Literal(token) => {
            Some(token)
        }
        SQLExpression::FunctionCall { name, .. } => Some(name),
        SQLExpression::UnaryOp { op, .. }
        | SQLExpression::BinaryOp { op, .. } => Some(op),
        SQLExpression::Vector(items) => {
            items.first().and_then(expression_token)
        }
        SQLExpression::InList { expr, .. }
        | SQLExpression::Between { expr, .. } => expression_token(expr),
        SQLExpression::SubQuery(select) => select.from.as_ref(),
    }
}

fn a_order_item_token(
    order_by: Option<
        &crate::langs::sql::ast::statements::query::SQLOrderByClause,
    >,
    index: usize,
) -> Option<&Token2> {
    order_by
        .and_then(|clause| clause.items.get(index))
        .and_then(|item| expression_token(&item.expr))
}

fn alter_action_token(action: &SQLAlterAction) -> Option<&Token2> {
    match action {
        SQLAlterAction::AddColumn(name, _)
        | SQLAlterAction::DropColumn(name)
        | SQLAlterAction::RenameTable(name) => Some(name),
        SQLAlterAction::RenameTo(name, _) => Some(name),
    }
}
