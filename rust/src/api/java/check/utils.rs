use super::*;

pub(super) fn mark(
    conditions: &mut [(bool, String)],
    index: usize,
    passed: bool,
) {
    if let Some((value, _)) = conditions.get_mut(index) {
        *value = passed;
    }
}

pub(super) fn main_body(class: &JavaClass) -> Option<&[JavaStatement]> {
    class.members.iter().find_map(|member| match member {
        JavaMember::Method(method) if method.name.text == "main" => {
            Some(method.body.as_slice())
        }
        _ => None,
    })
}

pub(super) fn method(class: &JavaClass, is_main: bool) -> Option<&JavaMethod> {
    class.members.iter().find_map(|member| match member {
        JavaMember::Method(value) if (value.name.text == "main") == is_main => {
            Some(value)
        }
        _ => None,
    })
}

pub(super) fn missing(diagnostics: &mut Diagnostics, message: &str) {
    diagnostics.record_again(message, None, None);
}

pub(super) fn mismatch(
    diagnostics: &mut Diagnostics,
    message: impl Into<String>,
    expr: Option<&JavaExpression>,
) {
    let (line, col) =
        expr.and_then(expression_location).unwrap_or((None, None));
    diagnostics.record_hard(message, line, col);
}

pub(super) fn mismatch_token(
    diagnostics: &mut Diagnostics,
    message: impl Into<String>,
    token: &Token2,
) {
    diagnostics.record_hard(message, Some(token.line), Some(token.col));
}

pub(super) fn return_value(
    statement: &JavaStatement,
) -> Option<&JavaExpression> {
    match statement {
        JavaStatement::Return(value) => value.as_ref(),
        _ => None,
    }
}

pub(super) fn compare_optional_expr(
    d: &mut Diagnostics,
    a: Option<&JavaExpression>,
    b: Option<&JavaExpression>,
) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => compare_expr(d, a, b),
        (None, None) => true,
        (Some(_), None) => {
            mismatch(d, "An expected expression is missing", None);
            false
        }
        (None, Some(b)) => {
            mismatch(d, "Found an unexpected expression", Some(b));
            false
        }
    }
}

pub(super) fn check_variable(
    d: &mut Diagnostics,
    a: &JavaStatement,
    b: &JavaStatement,
) -> bool {
    let (
        JavaStatement::LocalVariable { name: an, r#type: at, init: ai },
        JavaStatement::LocalVariable { name: bn, r#type: bt, init: bi },
    ) = (a, b)
    else {
        missing(d, "Expecting a variable declaration");
        return false;
    };
    let mut good = true;
    if an.text != bn.text {
        mismatch_token(
            d,
            format!("Expected variable `{}` but found `{}`", an.text, bn.text),
            bn,
        );
        good = false;
    }
    if at.text != bt.text {
        mismatch_token(
            d,
            format!("Expected type `{}` but found `{}`", at.text, bt.text),
            bt,
        );
        good = false;
    }
    good &= compare_optional_expr(d, ai.as_ref(), bi.as_ref());
    good
}

pub(super) fn check_print(
    d: &mut Diagnostics,
    a: &JavaStatement,
    b: &JavaStatement,
) -> bool {
    let (Some(ac), Some(bc)) = (as_call(a), as_call(b)) else {
        missing(d, "Expected a print call");
        return false;
    };
    if !is_print_name(&ac.name.text) {
        missing(d, "The generated statement is not a print call");
        return false;
    }
    if !is_print_name(&bc.name.text) {
        mismatch_token(d, "Expected print or println", &bc.name);
        return false;
    }
    if ac.arguments.len() != bc.arguments.len() {
        mismatch(
            d,
            "The print call has the wrong number of arguments",
            bc.arguments.first(),
        );
        return false;
    }
    ac.arguments.iter().zip(&bc.arguments).all(|(x, y)| compare_expr(d, x, y))
}

fn is_print_name(name: &str) -> bool {
    matches!(name, "print" | "println")
}

pub(super) fn as_call(statement: &JavaStatement) -> Option<&JavaMethodCall> {
    let JavaStatement::Expression(JavaExpression::MethodCall(call)) = statement
    else {
        return None;
    };
    Some(call)
}

pub(super) fn check_first_print(
    d: &mut Diagnostics,
    expected: &[JavaStatement],
    actual: &[JavaStatement],
) -> bool {
    match (expected.first(), actual.first()) {
        (Some(a), Some(b)) => check_print(d, a, b),
        _ => {
            mismatch(d, "Expected a statement in this block", None);
            false
        }
    }
}

pub(super) fn check_loop_update(
    d: &mut Diagnostics,
    expected: &[JavaStatement],
    actual: &[JavaStatement],
) -> bool {
    let expected_update = expected.iter().find_map(update_expression);
    let actual_update = actual.iter().find_map(update_expression);
    match (expected_update, actual_update) {
        (Some(expected), Some(actual)) => compare_expr(d, expected, actual),
        (_, None) => {
            let location = actual.last().and_then(statement_expression);
            mismatch(d, "Expected an update statement in the loop", location);
            false
        }
        (None, Some(actual)) => {
            mismatch(
                d,
                "The loop has an unexpected update statement",
                Some(actual),
            );
            false
        }
    }
}

fn statement_expression(statement: &JavaStatement) -> Option<&JavaExpression> {
    match statement {
        JavaStatement::Expression(expression) => Some(expression),
        JavaStatement::LocalVariable { init, .. } => init.as_ref(),
        JavaStatement::Return(expression) => expression.as_ref(),
        _ => None,
    }
}

fn update_expression(statement: &JavaStatement) -> Option<&JavaExpression> {
    let JavaStatement::Expression(expression) = statement else {
        return None;
    };
    match expression {
        JavaExpression::Assignment { .. } => Some(expression),
        JavaExpression::UnaryOp { op, .. }
            if matches!(op.text.as_str(), "++" | "--") =>
        {
            Some(expression)
        }
        _ => None,
    }
}

pub(super) fn statement_body(statement: &JavaStatement) -> &[JavaStatement] {
    match statement {
        JavaStatement::Block(items) => items,
        _ => std::slice::from_ref(statement),
    }
}

pub(super) fn find_statement(
    statements: &[JavaStatement],
    predicate: fn(&JavaStatement) -> bool,
) -> Option<&JavaStatement> {
    for statement in statements {
        if predicate(statement) {
            return Some(statement);
        }
        let nested = match statement {
            JavaStatement::Block(items) => Some(items.as_slice()),
            JavaStatement::If { then, r#else, .. } => {
                if let Some(found) =
                    find_statement(statement_body(then), predicate)
                {
                    return Some(found);
                }
                r#else.as_deref().map(statement_body)
            }
            JavaStatement::For { body, .. }
            | JavaStatement::ForEach { body, .. }
            | JavaStatement::While { body, .. }
            | JavaStatement::DoWhile { body, .. } => Some(statement_body(body)),
            _ => None,
        };
        if let Some(found) =
            nested.and_then(|items| find_statement(items, predicate))
        {
            return Some(found);
        }
    }
    None
}
pub(super) fn is_if(s: &JavaStatement) -> bool {
    matches!(s, JavaStatement::If { .. })
}
pub(super) fn is_for(s: &JavaStatement) -> bool {
    matches!(s, JavaStatement::For { .. } | JavaStatement::ForEach { .. })
}
pub(super) fn is_while(s: &JavaStatement) -> bool {
    matches!(s, JavaStatement::While { .. })
}
pub(super) fn is_do(s: &JavaStatement) -> bool {
    matches!(s, JavaStatement::DoWhile { .. })
}
pub(super) fn is_switch(s: &JavaStatement) -> bool {
    matches!(s, JavaStatement::Switch { .. })
}
pub(super) fn first_expr(
    statements: &[JavaStatement],
) -> Option<&JavaExpression> {
    statements.iter().find_map(|s| match s {
        JavaStatement::Expression(e) => Some(e),
        JavaStatement::LocalVariable { init: Some(e), .. }
        | JavaStatement::Return(Some(e)) => Some(e),
        _ => None,
    })
}

pub(super) fn compare_expr(
    d: &mut Diagnostics,
    a: &JavaExpression,
    b: &JavaExpression,
) -> bool {
    use JavaExpression as E;
    let same = match (a, b) {
        (E::Identifier(a), E::Identifier(b))
        | (E::Literal(a), E::Literal(b)) => a.text == b.text,
        (E::This, E::This) | (E::Super, E::Super) => true,
        (E::MethodCall(a), E::MethodCall(b)) => {
            a.name.text == b.name.text
                && a.arguments.len() == b.arguments.len()
                && a.arguments
                    .iter()
                    .zip(&b.arguments)
                    .all(|(x, y)| compare_expr(d, x, y))
        }
        (
            E::BinaryOp { op: ao, left: al, right: ar },
            E::BinaryOp { op: bo, left: bl, right: br },
        )
        | (
            E::Assignment { op: ao, left: al, right: ar },
            E::Assignment { op: bo, left: bl, right: br },
        ) => {
            ao.text == bo.text
                && compare_expr(d, al, bl)
                && compare_expr(d, ar, br)
        }
        (
            E::UnaryOp { op: ao, expr: ae, .. },
            E::UnaryOp { op: bo, expr: be, .. },
        ) => ao.text == bo.text && compare_expr(d, ae, be),
        _ => false,
    };
    if !same {
        mismatch(d, "The expression does not match the drill", Some(b));
    }
    same
}

pub(super) fn expression_location(
    e: &JavaExpression,
) -> Option<(Option<usize>, Option<usize>)> {
    use JavaExpression as E;
    let t: &Token2 = match e {
        E::Identifier(t) | E::Literal(t) => t,
        E::BinaryOp { op, .. }
        | E::Assignment { op, .. }
        | E::UnaryOp { op, .. } => op,
        E::MethodCall(call) => {
            return Some((Some(call.name.line), Some(call.name.col)));
        }
        E::FieldAccess(access) => return expression_location(&access.target),
        E::ArrayAccess(access) => return expression_location(&access.target),
        E::Cast { target_type, .. } => target_type,
        E::Ternary { cond, .. } => return expression_location(cond),
        E::New(value) => {
            return value.arguments.first().and_then(expression_location);
        }
        E::ArrayCreation(value) => {
            return value.dim.first().and_then(expression_location);
        }
        E::Lambda(value) => return match &value.body {
            crate::langs::java::ast::expression::JavaLambdaBody::Expression(
                e,
            ) => expression_location(e),
            _ => None,
        },
        E::AnonymousClass(_) | E::This | E::Super => return None,
    };
    Some((Some(t.line), Some(t.col)))
}
