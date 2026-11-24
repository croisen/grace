use oak_rust::ast::{Expr, Function, Item, Statement, Type};
use oak_rust::lexer::RustTokenType;

use crate::api::diagnostics::Diagnostics;

#[flutter_rust_bridge::frb(ignore)]
pub fn find_function<'a>(
    items: &'a [Item],
    name: &str,
) -> Option<&'a Function> {
    items.iter().find_map(|item| match item {
        Item::Function(function) if function.name.name == name => {
            Some(function.as_ref())
        }
        _ => None,
    })
}

#[flutter_rust_bridge::frb(ignore)]
pub fn find_non_main_function(items: &[Item]) -> Option<&Function> {
    items.iter().find_map(|item| match item {
        Item::Function(function) if function.name.name != "main" => {
            Some(function.as_ref())
        }
        _ => None,
    })
}

#[flutter_rust_bridge::frb(ignore)]
pub fn mark(conditions: &mut [(bool, String)], index: usize, result: bool) {
    if let Some((passed, _)) = conditions.get_mut(index) {
        *passed = result;
    }
}

#[flutter_rust_bridge::frb(ignore)]
pub fn check_let(
    diagnostics: &mut Diagnostics,
    expected: &Statement,
    actual: &Statement,
) -> bool {
    let Statement::Let {
        name: expected_name,
        ty: expected_type,
        expr: expected_expr,
        mutable: expected_mutable,
        ..
    } = expected
    else {
        return false;
    };
    let Statement::Let { name, ty, expr, mutable, .. } = actual else {
        diagnostics.record_again("Expecting a let declaration");
        return false;
    };

    let mut result = true;
    if name.name != expected_name.name {
        diagnostics.record_hard(format!(
            "Expected variable name `{}` but found `{}`",
            expected_name.name, name.name
        ));
        result = false;
    }
    if mutable != expected_mutable {
        diagnostics.record_hard(if *expected_mutable {
            "Expected a mutable variable declaration (`let mut`)"
        } else {
            "Expected an immutable variable declaration"
        });
        result = false;
    }
    if let Some(expected_type) = expected_type {
        if ty.as_ref() != Some(expected_type) {
            diagnostics.record_hard(format!(
                "Expected type `{}` but found `{}`",
                type_name(expected_type),
                ty.as_ref().map(type_name).unwrap_or("an inferred type")
            ));
            result = false;
        }
    }
    match (expected_expr, expr) {
        (Some(expected), Some(actual)) => {
            result &= compare_expr(diagnostics, expected, actual);
        }
        (Some(_), None) => {
            diagnostics.record_hard("Expected an initializer for the variable");
            result = false;
        }
        (None, Some(_)) => {
            diagnostics.record_good("The variable has an extra initializer");
        }
        (None, None) => {}
    }
    result
}

#[flutter_rust_bridge::frb(ignore)]
pub fn check_print(
    diagnostics: &mut Diagnostics,
    expected: &Statement,
    actual: &Statement,
) -> bool {
    let Some(expected_args) = print_args(expected) else {
        return false;
    };
    let Some(actual_args) = print_args(actual) else {
        diagnostics.record_again("Expected a `print!` or `println!` call");
        return false;
    };

    let expected_value = print_value(expected_args);
    let actual_value = print_value(actual_args);
    match (expected_value, actual_value) {
        (Some(expected), Some(actual)) => {
            compare_expr(diagnostics, expected, actual)
        }
        _ => {
            diagnostics.record_again("Could not read the printed value");
            false
        }
    }
}

#[flutter_rust_bridge::frb(ignore)]
pub fn compare_expr(
    diagnostics: &mut Diagnostics,
    expected: &Expr,
    actual: &Expr,
) -> bool {
    use Expr::*;
    match (unparen(expected), unparen(actual)) {
        (Ident(a), Ident(b)) if a.name == b.name => true,
        (Literal { value: a, .. }, Literal { value: b, .. }) if a == b => true,
        (Bool { value: a, .. }, Bool { value: b, .. }) if a == b => true,
        (Unary { op: aop, expr: a, .. }, Unary { op: bop, expr: b, .. }) => {
            if aop != bop {
                diagnostics.record_hard(format!(
                    "Expected unary operator `{}` but found `{}`",
                    token_name(*aop),
                    token_name(*bop)
                ));
                return false;
            }
            compare_expr(diagnostics, a, b)
        }
        (
            Binary { left: al, op: ao, right: ar, .. },
            Binary { left: bl, op: bo, right: br, .. },
        ) => compare_binary(diagnostics, al, *ao, ar, bl, *bo, br),
        (
            Call { callee: ac, args: aa, .. },
            Call { callee: bc, args: ba, .. },
        ) => {
            let mut result = compare_expr(diagnostics, ac, bc);
            if aa.len() != ba.len() {
                diagnostics.record_hard(format!(
                    "Expected {} call argument(s) but found {}",
                    aa.len(),
                    ba.len()
                ));
                return false;
            }
            for (expected, actual) in aa.iter().zip(ba) {
                result &= compare_expr(diagnostics, expected, actual);
            }
            result
        }
        (
            Field { receiver: ar, field: af, .. },
            Field { receiver: br, field: bf, .. },
        ) if af.name == bf.name => compare_expr(diagnostics, ar, br),
        (
            Index { receiver: ar, index: ai, .. },
            Index { receiver: br, index: bi, .. },
        ) => {
            compare_expr(diagnostics, ar, br)
                && compare_expr(diagnostics, ai, bi)
        }
        (Tuple { elements: a, .. }, Tuple { elements: b, .. })
        | (Array { elements: a, .. }, Array { elements: b, .. }) => {
            compare_expr_lists(diagnostics, a, b)
        }
        _ => {
            diagnostics.record_hard(format!(
                "Expected expression `{}` but found `{}`",
                expr_name(unparen(expected)),
                expr_name(unparen(actual))
            ));
            false
        }
    }
}

fn compare_binary(
    diagnostics: &mut Diagnostics,
    expected_left: &Expr,
    expected_op: RustTokenType,
    expected_right: &Expr,
    actual_left: &Expr,
    actual_op: RustTokenType,
    actual_right: &Expr,
) -> bool {
    if expected_op == actual_op {
        if expr_matches(expected_left, actual_left)
            && expr_matches(expected_right, actual_right)
        {
            return true;
        }
        if is_commutative(expected_op)
            && expr_matches(expected_left, actual_right)
            && expr_matches(expected_right, actual_left)
        {
            return true;
        }
        let _ = compare_expr(diagnostics, expected_left, actual_left);
        let _ = compare_expr(diagnostics, expected_right, actual_right);
        return false;
    }

    if inverse_comparison(expected_op) == Some(actual_op)
        && expr_matches(expected_left, actual_right)
        && expr_matches(expected_right, actual_left)
    {
        diagnostics.record_good(
            "The comparison is equivalent with its operands reversed",
        );
        return true;
    }

    diagnostics.record_hard(format!(
        "Expected binary operator `{}` but found `{}`",
        token_name(expected_op),
        token_name(actual_op)
    ));
    false
}

fn compare_expr_lists(
    diagnostics: &mut Diagnostics,
    expected: &[Expr],
    actual: &[Expr],
) -> bool {
    if expected.len() != actual.len() {
        diagnostics
            .record_hard("Expression contains a different number of elements");
        return false;
    }
    expected
        .iter()
        .zip(actual)
        .fold(true, |ok, (a, b)| compare_expr(diagnostics, a, b) && ok)
}

fn expr_matches(expected: &Expr, actual: &Expr) -> bool {
    use Expr::*;
    match (unparen(expected), unparen(actual)) {
        (Ident(a), Ident(b)) => a.name == b.name,
        (Literal { value: a, .. }, Literal { value: b, .. }) => a == b,
        (Bool { value: a, .. }, Bool { value: b, .. }) => a == b,
        (Unary { op: aop, expr: a, .. }, Unary { op: bop, expr: b, .. }) => {
            aop == bop && expr_matches(a, b)
        }
        (
            Binary { left: al, op: ao, right: ar, .. },
            Binary { left: bl, op: bo, right: br, .. },
        ) => {
            ao == bo
                && ((expr_matches(al, bl) && expr_matches(ar, br))
                    || (is_commutative(*ao)
                        && expr_matches(al, br)
                        && expr_matches(ar, bl)))
        }
        (
            Call { callee: ac, args: aa, .. },
            Call { callee: bc, args: ba, .. },
        ) => {
            expr_matches(ac, bc)
                && aa.len() == ba.len()
                && aa.iter().zip(ba).all(|(a, b)| expr_matches(a, b))
        }
        (
            Field { receiver: ar, field: af, .. },
            Field { receiver: br, field: bf, .. },
        ) => af.name == bf.name && expr_matches(ar, br),
        (
            Index { receiver: ar, index: ai, .. },
            Index { receiver: br, index: bi, .. },
        ) => expr_matches(ar, br) && expr_matches(ai, bi),
        (Tuple { elements: a, .. }, Tuple { elements: b, .. })
        | (Array { elements: a, .. }, Array { elements: b, .. }) => {
            a.len() == b.len()
                && a.iter().zip(b).all(|(a, b)| expr_matches(a, b))
        }
        _ => false,
    }
}

fn print_args(statement: &Statement) -> Option<&[Expr]> {
    let Statement::ExprStmt { expr, .. } = statement else { return None };
    let Expr::Call { callee, args, .. } = expr else { return None };
    let Expr::Ident(name) = callee.as_ref() else { return None };
    matches!(name.name.as_str(), "print" | "println").then_some(args)
}

fn print_value(args: &[Expr]) -> Option<&Expr> {
    match args {
        [single] => Some(single),
        [Expr::Literal { value, .. }, value_expr]
            if is_format_placeholder(value) =>
        {
            Some(value_expr)
        }
        _ => None,
    }
}

fn is_format_placeholder(value: &str) -> bool {
    matches!(value.trim(), "\"{}\"" | "r#\"{}\"#")
}

fn unparen(mut expr: &Expr) -> &Expr {
    while let Expr::Paren { expr: inner, .. } = expr {
        expr = inner;
    }
    expr
}

fn is_commutative(op: RustTokenType) -> bool {
    matches!(
        op,
        RustTokenType::Plus
            | RustTokenType::Star
            | RustTokenType::EqEq
            | RustTokenType::Ne
            | RustTokenType::AndAnd
            | RustTokenType::OrOr
            | RustTokenType::Ampersand
            | RustTokenType::Pipe
            | RustTokenType::Caret
    )
}

fn inverse_comparison(op: RustTokenType) -> Option<RustTokenType> {
    Some(match op {
        RustTokenType::Lt => RustTokenType::Gt,
        RustTokenType::Le => RustTokenType::Ge,
        RustTokenType::Gt => RustTokenType::Lt,
        RustTokenType::Ge => RustTokenType::Le,
        _ => return None,
    })
}

fn type_name(ty: &Type) -> &str {
    match ty {
        Type::Path(name) => name,
        Type::Reference { .. } => "reference",
        Type::Tuple(_) => "tuple",
        Type::Array { .. } => "array",
        Type::Slice(_) => "slice",
        Type::Fn { .. } => "function pointer",
        Type::Infer => "inferred type",
    }
}

fn token_name(token: RustTokenType) -> &'static str {
    match token {
        RustTokenType::Plus => "+",
        RustTokenType::Minus => "-",
        RustTokenType::Star => "*",
        RustTokenType::Slash => "/",
        RustTokenType::Percent => "%",
        RustTokenType::EqEq => "==",
        RustTokenType::Ne => "!=",
        RustTokenType::Lt => "<",
        RustTokenType::Le => "<=",
        RustTokenType::Gt => ">",
        RustTokenType::Ge => ">=",
        RustTokenType::AndAnd => "&&",
        RustTokenType::OrOr => "||",
        RustTokenType::Assign => "=",
        RustTokenType::PlusAssign => "+=",
        RustTokenType::MinusAssign => "-=",
        RustTokenType::DotDot => "..",
        RustTokenType::DotDotEq => "..=",
        RustTokenType::Bang => "!",
        _ => "operator",
    }
}

fn expr_name(expr: &Expr) -> &'static str {
    match expr {
        Expr::Ident(_) => "identifier",
        Expr::Literal { .. } => "literal",
        Expr::Bool { .. } => "boolean",
        Expr::Unary { .. } => "unary expression",
        Expr::Binary { .. } => "binary expression",
        Expr::Call { .. } => "function or macro call",
        Expr::Field { .. } => "field access",
        Expr::Index { .. } => "index expression",
        Expr::Paren { .. } => "parenthesized expression",
        Expr::Block(_) => "block",
        Expr::If { .. } => "if expression",
        Expr::While { .. } => "while loop",
        Expr::For { .. } => "for loop",
        Expr::Loop { .. } => "loop expression",
        Expr::Match { .. } => "match expression",
        Expr::Tuple { .. } => "tuple",
        Expr::Array { .. } => "array",
        Expr::Struct { .. } => "struct expression",
    }
}

fn function_body_tokens(source: &str, name: &str) -> Option<Vec<String>> {
    let tokens = tokenize(source);
    let start = tokens
        .windows(2)
        .position(|pair| pair[0] == "fn" && pair[1] == name)?;
    let params_open = start + 2;
    if tokens.get(params_open)?.as_str() != "(" {
        return None;
    }
    let params_close = matching_token(&tokens, params_open, "(", ")")?;
    let body_open = find_top_level(&tokens, params_close + 1, "{")?;
    let (body, _) = token_block(&tokens, body_open)?;
    Some(body)
}

#[flutter_rust_bridge::frb(ignore)]
pub fn main_tokens(source: &str) -> Option<Vec<String>> {
    function_body_tokens(source, "main")
}

#[flutter_rust_bridge::frb(ignore)]
pub fn split_statements(tokens: &[String]) -> Vec<Vec<String>> {
    let mut statements = Vec::new();
    let mut start = 0;
    let mut paren = 0usize;
    let mut bracket = 0usize;
    let mut brace = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match token.as_str() {
            "(" => paren += 1,
            ")" => paren = paren.saturating_sub(1),
            "[" => bracket += 1,
            "]" => bracket = bracket.saturating_sub(1),
            "{" => brace += 1,
            "}" => brace = brace.saturating_sub(1),
            ";" if paren == 0 && bracket == 0 && brace == 0 => {
                statements.push(tokens[start..index].to_vec());
                start = index + 1;
            }
            _ => {}
        }
    }
    if start < tokens.len() {
        statements.push(tokens[start..].to_vec());
    }
    statements
}

#[flutter_rust_bridge::frb(ignore)]
pub fn token_block(
    tokens: &[String],
    open: usize,
) -> Option<(Vec<String>, usize)> {
    if tokens.get(open)?.as_str() != "{" {
        return None;
    }
    let close = matching_token(tokens, open, "{", "}")?;
    Some((tokens[open + 1..close].to_vec(), close + 1))
}

#[flutter_rust_bridge::frb(ignore)]
pub fn open_brace_after(tokens: &[String], start: usize) -> Option<usize> {
    find_top_level(tokens, start, "{")
}

#[flutter_rust_bridge::frb(ignore)]
pub fn compare_token_expr(
    diagnostics: &mut Diagnostics,
    expected: &[String],
    actual: &[String],
) -> bool {
    if token_expr_matches(expected, actual) {
        return true;
    }
    diagnostics.record_hard(format!(
        "Expected expression `{}` but found `{}`",
        expected.join(" "),
        actual.join(" ")
    ));
    false
}

#[flutter_rust_bridge::frb(ignore)]
pub fn compare_token_condition(
    diagnostics: &mut Diagnostics,
    expected: &[String],
    actual: &[String],
) -> bool {
    if token_expr_matches(expected, actual) {
        return true;
    }
    if reversed_comparison(expected, actual) {
        diagnostics.record_good(
            "The comparison is equivalent with its operands reversed",
        );
        return true;
    }
    compare_token_expr(diagnostics, expected, actual)
}

#[flutter_rust_bridge::frb(ignore)]
pub fn check_token_let(
    diagnostics: &mut Diagnostics,
    expected: &[String],
    actual: &[String],
) -> bool {
    let expected_eq = expected.iter().position(|t| t == "=");
    let actual_eq = actual.iter().position(|t| t == "=");
    let (Some(expected_eq), Some(actual_eq)) = (expected_eq, actual_eq) else {
        diagnostics.record_again("Expecting an initialized `let` declaration");
        return false;
    };
    let mut result = true;
    let expected_head = &expected[..expected_eq];
    let actual_head = &actual[..actual_eq];
    if expected_head.first().map(String::as_str) != Some("let")
        || actual_head.first().map(String::as_str) != Some("let")
    {
        diagnostics.record_again("Expecting a `let` declaration");
        return false;
    }
    let expected_mutable =
        expected_head.get(1).map(String::as_str) == Some("mut");
    let actual_mutable = actual_head.get(1).map(String::as_str) == Some("mut");
    if expected_mutable != actual_mutable {
        diagnostics
            .record_hard("The variable mutability does not match the drill");
        result = false;
    }
    let expected_name_index = if expected_mutable { 2 } else { 1 };
    let actual_name_index = if actual_mutable { 2 } else { 1 };
    if expected_head.get(expected_name_index)
        != actual_head.get(actual_name_index)
    {
        diagnostics.record_hard(format!(
            "Expected variable `{}` but found `{}`",
            expected_head
                .get(expected_name_index)
                .map(String::as_str)
                .unwrap_or(""),
            actual_head
                .get(actual_name_index)
                .map(String::as_str)
                .unwrap_or("")
        ));
        result = false;
    }
    if expected_head.get(expected_name_index + 1..)
        != actual_head.get(actual_name_index + 1..)
    {
        diagnostics.record_hard(
            "The variable type annotation does not match the drill",
        );
        result = false;
    }
    result &= compare_token_expr(
        diagnostics,
        &expected[expected_eq + 1..],
        &actual[actual_eq + 1..],
    );
    result
}

#[flutter_rust_bridge::frb(ignore)]
pub fn check_token_print(
    diagnostics: &mut Diagnostics,
    expected: &[String],
    actual: &[String],
) -> bool {
    let (Some(expected_args), Some(actual_args)) =
        (print_call_args(expected), print_call_args(actual))
    else {
        diagnostics.record_again("Expected a `print!` or `println!` call");
        return false;
    };
    let Some(expected_value) = print_token_value(&expected_args) else {
        diagnostics.record_again("Could not read the expected printed value");
        return false;
    };
    let Some(actual_value) = print_token_value(&actual_args) else {
        diagnostics.record_again("Could not read the printed value");
        return false;
    };
    compare_token_expr(diagnostics, expected_value, actual_value)
}

fn tokenize(source: &str) -> Vec<String> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if c == '/' && bytes.get(i + 1) == Some(&b'/') {
            i += 2;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && bytes.get(i + 1) == Some(&b'*') {
            i += 2;
            let mut depth = 1usize;
            while i < bytes.len() && depth > 0 {
                if bytes.get(i..i + 2) == Some(b"/*") {
                    depth += 1;
                    i += 2;
                } else if bytes.get(i..i + 2) == Some(b"*/") {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        if c == 'r' && bytes.get(i + 1) == Some(&b'"') {
            let start = i;
            i += 2;
            while i < bytes.len() && bytes[i] != b'"' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            tokens.push(source[start..i].to_string());
            continue;
        }
        if c == '"' || (c == '\'' && bytes.get(i + 2) == Some(&b'\'')) {
            let quote = bytes[i];
            let start = i;
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'\\' {
                    i = (i + 2).min(bytes.len());
                } else if bytes[i] == quote {
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
            tokens.push(source[start..i].to_string());
            continue;
        }
        if c.is_ascii_alphanumeric() || c == '_' {
            let start = i;
            i += 1;
            while i < bytes.len()
                && ((bytes[i] as char).is_ascii_alphanumeric()
                    || bytes[i] == b'_')
            {
                i += 1;
            }
            tokens.push(source[start..i].to_string());
            continue;
        }
        let pair = bytes.get(i..i + 3).unwrap_or(&[]);
        if [b"..=".as_slice(), b"<<=".as_slice(), b">>=".as_slice()]
            .contains(&pair)
        {
            tokens.push(source[i..i + 3].to_string());
            i += 3;
            continue;
        }
        let pair = bytes.get(i..i + 2).unwrap_or(&[]);
        if [
            b"..".as_slice(),
            b"=>".as_slice(),
            b"==".as_slice(),
            b"!=".as_slice(),
            b"<=".as_slice(),
            b">=".as_slice(),
            b"&&".as_slice(),
            b"||".as_slice(),
            b"+=".as_slice(),
            b"-=".as_slice(),
            b"*=".as_slice(),
            b"/=".as_slice(),
            b"->".as_slice(),
            b"::".as_slice(),
            b"<<".as_slice(),
            b">>".as_slice(),
        ]
        .contains(&pair)
        {
            tokens.push(source[i..i + 2].to_string());
            i += 2;
            continue;
        }
        tokens.push(c.to_string());
        i += 1;
    }
    tokens
}

fn matching_token(
    tokens: &[String],
    open: usize,
    left: &str,
    right: &str,
) -> Option<usize> {
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate().skip(open) {
        if token == left {
            depth += 1;
        } else if token == right {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(index);
            }
        }
    }
    None
}

fn find_top_level(
    tokens: &[String],
    start: usize,
    target: &str,
) -> Option<usize> {
    let mut paren = 0usize;
    let mut bracket = 0usize;
    for (index, token) in tokens.iter().enumerate().skip(start) {
        if token == target && paren == 0 && bracket == 0 {
            return Some(index);
        }
        match token.as_str() {
            "(" => paren += 1,
            ")" => paren = paren.saturating_sub(1),
            "[" => bracket += 1,
            "]" => bracket = bracket.saturating_sub(1),
            _ => {}
        }
    }
    None
}

#[flutter_rust_bridge::frb(ignore)]
pub fn split_tokens(tokens: &[String], separator: &str) -> Vec<Vec<String>> {
    if tokens.is_empty() {
        return Vec::new();
    }
    let mut parts = Vec::new();
    let mut start = 0;
    let mut paren = 0usize;
    let mut bracket = 0usize;
    let mut brace = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        if token == separator && paren == 0 && bracket == 0 && brace == 0 {
            parts.push(tokens[start..index].to_vec());
            start = index + 1;
            continue;
        }
        match token.as_str() {
            "(" => paren += 1,
            ")" => paren = paren.saturating_sub(1),
            "[" => bracket += 1,
            "]" => bracket = bracket.saturating_sub(1),
            "{" => brace += 1,
            "}" => brace = brace.saturating_sub(1),
            _ => {}
        }
    }
    parts.push(tokens[start..].to_vec());
    if parts.last().is_some_and(Vec::is_empty) {
        parts.pop();
    }
    parts
}

fn token_expr_matches(expected: &[String], actual: &[String]) -> bool {
    let expected = strip_outer_parens(expected);
    let actual = strip_outer_parens(actual);
    if expected == actual {
        return true;
    }
    let (Some((el, eo, er)), Some((al, ao, ar))) =
        (top_level_binary(expected), top_level_binary(actual))
    else {
        return false;
    };
    eo == ao
        && ((token_expr_matches(el, al) && token_expr_matches(er, ar))
            || (matches!(eo, "+" | "*" | "==" | "!=" | "&&" | "||")
                && token_expr_matches(el, ar)
                && token_expr_matches(er, al)))
}

fn reversed_comparison(expected: &[String], actual: &[String]) -> bool {
    let (Some((el, eo, er)), Some((al, ao, ar))) = (
        top_level_binary(strip_outer_parens(expected)),
        top_level_binary(strip_outer_parens(actual)),
    ) else {
        return false;
    };
    let inverse = match eo {
        "<" => ">",
        "<=" => ">=",
        ">" => "<",
        ">=" => "<=",
        _ => return false,
    };
    inverse == ao && token_expr_matches(el, ar) && token_expr_matches(er, al)
}

fn top_level_binary(tokens: &[String]) -> Option<(&[String], &str, &[String])> {
    let mut paren = 0usize;
    let mut bracket = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match token.as_str() {
            "(" => paren += 1,
            ")" => paren = paren.saturating_sub(1),
            "[" => bracket += 1,
            "]" => bracket = bracket.saturating_sub(1),
            "==" | "!=" | "<" | "<=" | ">" | ">=" | "+" | "-" | "*" | "/"
            | "&&" | "||"
                if paren == 0
                    && bracket == 0
                    && index > 0
                    && index + 1 < tokens.len() =>
            {
                return Some((&tokens[..index], token, &tokens[index + 1..]));
            }
            _ => {}
        }
    }
    None
}

fn strip_outer_parens(mut tokens: &[String]) -> &[String] {
    while tokens.first().map(String::as_str) == Some("(")
        && tokens.last().map(String::as_str) == Some(")")
        && matching_token(tokens, 0, "(", ")") == Some(tokens.len() - 1)
    {
        tokens = &tokens[1..tokens.len() - 1];
    }
    tokens
}

fn print_call_args(tokens: &[String]) -> Option<Vec<Vec<String>>> {
    let tokens = tokens.strip_suffix(&[";".to_string()]).unwrap_or(tokens);
    if !matches!(tokens.first().map(String::as_str), Some("print" | "println"))
        || tokens.get(1).map(String::as_str) != Some("!")
        || tokens.get(2).map(String::as_str) != Some("(")
        || matching_token(tokens, 2, "(", ")")? != tokens.len() - 1
    {
        return None;
    }
    Some(split_tokens(&tokens[3..tokens.len() - 1], ","))
}

fn print_token_value(args: &[Vec<String>]) -> Option<&[String]> {
    match args {
        [single] => Some(single),
        [format, value] if format == &["\"{}\"".to_string()] => Some(value),
        _ => None,
    }
}
