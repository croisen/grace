use super::*;

pub(super) fn check(
    concept: SqlConcept,
    conditions: &mut [(bool, String)],
    diagnostics: &mut Diagnostics,
    expected: &SQLSelect,
    actual: &SQLSelect,
) {
    let mut index = 0;
    let compare_items =
        matches!(concept, SqlConcept::Select | SqlConcept::GroupBy);
    if compare_items {
        for item in 0..expected.items.len().max(actual.items.len()) {
            let (Some(e), Some(a)) =
                (expected.items.get(item), actual.items.get(item))
            else {
                check_part(
                    conditions,
                    diagnostics,
                    index,
                    false,
                    "A selected item is missing or extra",
                    actual.items.get(item).and_then(select_item_token),
                );
                index += 1;
                continue;
            };
            check_part(
                conditions,
                diagnostics,
                index,
                select_item_eq(e, a),
                "The selected item does not match",
                select_item_token(a),
            );
            index += 1;
        }
    } else if matches!(
        concept,
        SqlConcept::Where | SqlConcept::OrderBy | SqlConcept::Join
    ) {
        // These drills start with SELECT * and one source table.
        for i in 0..expected.items.len().max(actual.items.len()) {
            let (Some(e), Some(a)) =
                (expected.items.get(i), actual.items.get(i))
            else {
                check_part(
                    conditions,
                    diagnostics,
                    index,
                    false,
                    "The SELECT list does not match",
                    actual.items.get(i).and_then(select_item_token),
                );
                index += 1;
                continue;
            };
            check_part(
                conditions,
                diagnostics,
                index,
                select_item_eq(e, a),
                "The SELECT list does not match",
                select_item_token(a),
            );
            index += 1;
        }
    }

    if concept == SqlConcept::Having {
        match (expected.items.first(), actual.items.first()) {
            (Some(e), Some(a)) => {
                check_part(
                    conditions,
                    diagnostics,
                    0,
                    select_item_eq(e, a),
                    "The group key projection does not match",
                    select_item_token(a),
                );
            }
            _ => check_part(
                conditions,
                diagnostics,
                0,
                false,
                "The group key projection is missing",
                None,
            ),
        }
        match (expected.items.get(1), actual.items.get(1)) {
            (
                Some(SQLSelectItem::Expr(e, Some(ealias))),
                Some(SQLSelectItem::Expr(a, Some(aalias))),
            ) => {
                check_part(
                    conditions,
                    diagnostics,
                    1,
                    expr_eq(e, a),
                    "The aggregate projection does not match",
                    expression_token(a),
                );
                check_part(
                    conditions,
                    diagnostics,
                    2,
                    expr_eq(ealias, aalias),
                    "The aggregate alias does not match",
                    expression_token(aalias),
                );
            }
            (
                Some(SQLSelectItem::Expr(e, None)),
                Some(SQLSelectItem::Expr(a, None)),
            ) => {
                check_part(
                    conditions,
                    diagnostics,
                    1,
                    expr_eq(e, a),
                    "The aggregate projection does not match",
                    expression_token(a),
                );
                check_part(
                    conditions,
                    diagnostics,
                    2,
                    true,
                    "The aggregate alias does not match",
                    None,
                );
            }
            (_, Some(item)) => {
                check_part(
                    conditions,
                    diagnostics,
                    1,
                    false,
                    "The aggregate projection does not match",
                    select_item_token(item),
                );
                check_part(
                    conditions,
                    diagnostics,
                    2,
                    false,
                    "The aggregate alias does not match",
                    select_item_token(item),
                );
            }
            _ => {
                missing_clause(
                    diagnostics,
                    conditions,
                    1,
                    "The aggregate projection is missing",
                );
                missing_clause(
                    diagnostics,
                    conditions,
                    2,
                    "The aggregate alias is missing",
                );
            }
        }
        index = 3;
    }

    if concept == SqlConcept::Aggregate {
        match (expected.items.first(), actual.items.first()) {
            (
                Some(SQLSelectItem::Expr(
                    SQLExpression::FunctionCall { name: en, args: ea },
                    ealias,
                )),
                Some(SQLSelectItem::Expr(
                    SQLExpression::FunctionCall { name: an, args: aa },
                    aalias,
                )),
            ) => {
                check_part(
                    conditions,
                    diagnostics,
                    index,
                    token_eq(en, an),
                    "The aggregate function does not match",
                    Some(an),
                );
                index += 1;
                if let (Some(e), Some(a)) = (ea.first(), aa.first()) {
                    check_part(
                        conditions,
                        diagnostics,
                        index,
                        expr_eq(e, a),
                        "The aggregate argument does not match",
                        expression_token(a),
                    );
                } else {
                    check_part(
                        conditions,
                        diagnostics,
                        index,
                        false,
                        "The aggregate argument is missing",
                        aa.first().and_then(expression_token),
                    );
                }
                index += 1;
                let alias_ok = match (ealias, aalias) {
                    (Some(e), Some(a)) => expr_eq(e, a),
                    (None, None) => true,
                    _ => false,
                };
                check_part(
                    conditions,
                    diagnostics,
                    index,
                    alias_ok,
                    "The aggregate alias does not match",
                    aalias.as_ref().and_then(expression_token),
                );
                index += 1;
            }
            (_, Some(item)) => {
                check_part(
                    conditions,
                    diagnostics,
                    index,
                    false,
                    "Expected an aggregate function",
                    select_item_token(item),
                );
                index += 3;
            }
            _ => {
                missing_clause(
                    diagnostics,
                    conditions,
                    index,
                    "The aggregate expression is missing",
                );
                index += 3;
            }
        }
    }

    if matches!(
        concept,
        SqlConcept::Select
            | SqlConcept::Where
            | SqlConcept::OrderBy
            | SqlConcept::GroupBy
            | SqlConcept::Having
            | SqlConcept::Join
            | SqlConcept::Aggregate
    ) {
        let from_ok = match (&expected.from, &actual.from) {
            (Some(e), Some(a)) => token_eq(e, a),
            (None, None) => true,
            _ => false,
        };
        check_part(
            conditions,
            diagnostics,
            index,
            from_ok,
            "The FROM table does not match",
            actual.from.as_ref(),
        );
        index += 1;
    }

    match concept {
        SqlConcept::Where => match (&expected.r#where, &actual.r#where) {
            (Some(e), Some(a)) => check_predicate(
                conditions,
                diagnostics,
                &mut index,
                e,
                a,
                "WHERE",
            ),
            (_, None) => missing_clause(
                diagnostics,
                conditions,
                index,
                "The WHERE clause is missing",
            ),
            (None, _) => check_part(
                conditions,
                diagnostics,
                index,
                false,
                "Unexpected WHERE clause",
                actual.r#where.as_ref().and_then(expression_token),
            ),
        },
        SqlConcept::OrderBy => match (&expected.order_by, &actual.order_by) {
            (Some(e), Some(a)) => {
                for i in 0..e.items.len().max(a.items.len()) {
                    let (Some(e), Some(a)) = (e.items.get(i), a.items.get(i))
                    else {
                        check_part(
                            conditions,
                            diagnostics,
                            index,
                            false,
                            "An ORDER BY term is missing or extra",
                            a_order_item_token(actual.order_by.as_ref(), i),
                        );
                        index += 2;
                        continue;
                    };
                    check_part(
                        conditions,
                        diagnostics,
                        index,
                        expr_eq(&e.expr, &a.expr),
                        "The ORDER BY expression does not match",
                        expression_token(&a.expr),
                    );
                    index += 1;
                    check_part(
                        conditions,
                        diagnostics,
                        index,
                        debug_eq(&e.direction, &a.direction),
                        "The sort direction does not match",
                        expression_token(&a.expr),
                    );
                    index += 1;
                }
            }
            (_, None) => missing_clause(
                diagnostics,
                conditions,
                index,
                "The ORDER BY clause is missing",
            ),
            (None, _) => check_part(
                conditions,
                diagnostics,
                index,
                false,
                "Unexpected ORDER BY clause",
                None,
            ),
        },
        SqlConcept::GroupBy | SqlConcept::Having => {
            match (&expected.group_by, &actual.group_by) {
                (Some(e), Some(a)) => {
                    for i in 0..e.columns.len().max(a.columns.len()) {
                        let (Some(e), Some(a)) =
                            (e.columns.get(i), a.columns.get(i))
                        else {
                            check_part(
                                conditions,
                                diagnostics,
                                index,
                                false,
                                "A GROUP BY column is missing or extra",
                                actual
                                    .group_by
                                    .as_ref()
                                    .and_then(|g| g.columns.get(i))
                                    .and_then(expression_token),
                            );
                            index += 1;
                            continue;
                        };
                        check_part(
                            conditions,
                            diagnostics,
                            index,
                            expr_eq(e, a),
                            "The GROUP BY expression does not match",
                            expression_token(a),
                        );
                        index += 1;
                    }
                }
                (_, None) => missing_clause(
                    diagnostics,
                    conditions,
                    index,
                    "The GROUP BY clause is missing",
                ),
                (None, _) => check_part(
                    conditions,
                    diagnostics,
                    index,
                    false,
                    "Unexpected GROUP BY clause",
                    None,
                ),
            }
        }
        _ => {}
    }

    if concept == SqlConcept::Having {
        match (&expected.having, &actual.having) {
            (Some(e), Some(a)) => check_predicate(
                conditions,
                diagnostics,
                &mut index,
                &e.condition,
                &a.condition,
                "HAVING",
            ),
            (_, None) => missing_clause(
                diagnostics,
                conditions,
                index,
                "The HAVING clause is missing",
            ),
            (None, _) => check_part(
                conditions,
                diagnostics,
                index,
                false,
                "Unexpected HAVING clause",
                actual
                    .having
                    .as_ref()
                    .and_then(|h| expression_token(&h.condition)),
            ),
        }
    }

    if concept == SqlConcept::Join {
        for i in 0..expected.joins.len().max(actual.joins.len()) {
            let (Some(e), Some(a)) =
                (expected.joins.get(i), actual.joins.get(i))
            else {
                check_part(
                    conditions,
                    diagnostics,
                    index,
                    false,
                    "A JOIN clause is missing or extra",
                    actual.joins.get(i).map(|j| &j.table),
                );
                index += 1;
                continue;
            };
            let join_ok =
                debug_eq(&e.join, &a.join) && token_eq(&e.table, &a.table);
            check_part(
                conditions,
                diagnostics,
                index,
                join_ok,
                "The joined table or join type does not match",
                Some(&a.table),
            );
            index += 1;
            match (&e.on, &a.on) {
                (Some(e), Some(a)) => check_predicate(
                    conditions,
                    diagnostics,
                    &mut index,
                    e,
                    a,
                    "JOIN",
                ),
                (_, None) => missing_clause(
                    diagnostics,
                    conditions,
                    index,
                    "The JOIN ON condition is missing",
                ),
                (None, _) => check_part(
                    conditions,
                    diagnostics,
                    index,
                    false,
                    "Unexpected JOIN ON condition",
                    a.on.as_ref().and_then(expression_token),
                ),
            }
        }
    }
}
