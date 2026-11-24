use crate::api::diagnostics::Diagnostics;
use crate::core::parser::pratt::Token2;
use crate::langs::rust::ast::RustAst;
use crate::langs::rust::ast::expression::RustExpression;
use crate::langs::rust::ast::item::RustItem;
use crate::langs::rust::ast::item::function::RustFunction;
use crate::langs::rust::ast::statement::RustStatement;

#[flutter_rust_bridge::frb(ignore)]
pub fn ast(source: &str) -> (Vec<RustAst>, Diagnostics) {
    let output = RustAst::new(source);
    (output.root, Diagnostics::new(output.errors))
}

#[flutter_rust_bridge::frb(ignore)]
pub fn find_function<'a>(
    items: &'a [RustAst],
    name: &str,
) -> Option<&'a RustFunction> {
    items.iter().find_map(|node| match &node.item {
        RustItem::Function(function) if function.name.text == name => {
            Some(function)
        }
        _ => None,
    })
}

#[flutter_rust_bridge::frb(ignore)]
pub fn find_non_main_function(items: &[RustAst]) -> Option<&RustFunction> {
    items.iter().find_map(|node| match &node.item {
        RustItem::Function(function) if function.name.text != "main" => {
            Some(function)
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
    expected: &RustStatement,
    actual: &RustStatement,
) -> bool {
    let RustStatement::Let {
        mutable: expected_mut,
        name: expected_name,
        r#type: expected_type,
        init: expected_init,
    } = expected
    else {
        return false;
    };
    let RustStatement::Let { mutable, name, r#type, init } = actual else {
        diagnostics.record_again("Expecting a let declaration", None, None);
        return false;
    };
    let (line, col) = (Some(name.line), Some(name.col));
    let mut ok = true;
    if name.text != expected_name.text {
        diagnostics.record_hard(
            format!(
                "Expected variable `{}` but found `{}`",
                expected_name.text, name.text
            ),
            line,
            col,
        );
        ok = false;
    }
    if mutable != expected_mut {
        diagnostics.record_hard(
            "The variable mutability does not match the drill",
            line,
            col,
        );
        ok = false;
    }
    if !rust_type_matches(expected_type.as_ref(), r#type.as_ref()) {
        diagnostics.record_hard(
            "The variable type does not match the drill",
            line,
            col,
        );
        ok = false;
    }
    match (expected_init, init) {
        (Some(a), Some(b)) => ok &= compare_expr(diagnostics, a, b),
        (Some(_), None) => {
            diagnostics.record_hard(
                "Expected an initializer for the variable",
                line,
                col,
            );
            ok = false;
        }
        (None, Some(_)) => diagnostics.record_good(
            "The variable has an extra initializer",
            line,
            col,
        ),
        (None, None) => (),
    }
    ok
}

pub(super) fn rust_type_matches(
    expected: Option<&crate::langs::rust::ast::r#type::RustType>,
    actual: Option<&crate::langs::rust::ast::r#type::RustType>,
) -> bool {
    use crate::langs::rust::ast::r#type::RustType as T;
    match (expected, actual) {
        (None, None) => true,
        (Some(T::Path(a)), Some(T::Path(b))) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| a.text == b.text)
        }
        (Some(T::Reference(al, am, at)), Some(T::Reference(bl, bm, bt))) => {
            am == bm
                && al.as_ref().map(|l| l.0.text.as_str())
                    == bl.as_ref().map(|l| l.0.text.as_str())
                && rust_type_matches(Some(at), Some(bt))
        }
        (Some(T::Tuple(a)), Some(T::Tuple(b))) => {
            a.len() == b.len()
                && a.iter()
                    .zip(b)
                    .all(|(a, b)| rust_type_matches(Some(a), Some(b)))
        }
        (Some(T::Slice(a)), Some(T::Slice(b))) => {
            rust_type_matches(Some(a), Some(b))
        }
        (Some(T::Array(at, ae)), Some(T::Array(bt, be))) => {
            rust_type_matches(Some(at), Some(bt)) && expression_matches(ae, be)
        }
        (Some(T::Fn(aa, ar)), Some(T::Fn(ba, br))) => {
            aa.len() == ba.len()
                && aa
                    .iter()
                    .zip(ba)
                    .all(|(a, b)| rust_type_matches(Some(a), Some(b)))
                && rust_type_matches(ar.as_deref(), br.as_deref())
        }
        (Some(T::Inferred), Some(T::Inferred)) => true,
        _ => format!("{expected:?}") == format!("{actual:?}"),
    }
}

pub(super) fn rust_type_location(
    r#type: &crate::langs::rust::ast::r#type::RustType,
) -> (Option<usize>, Option<usize>) {
    use crate::langs::rust::ast::r#type::RustType as T;
    let token = match r#type {
        T::Path(path) => path.first(),
        T::Reference(_, _, inner) | T::Slice(inner) => {
            return rust_type_location(inner);
        }
        T::Tuple(items) => {
            return items
                .first()
                .map(rust_type_location)
                .unwrap_or((None, None));
        }
        T::Array(inner, _) => return rust_type_location(inner),
        T::Fn(args, ret) => {
            return args
                .first()
                .map(rust_type_location)
                .or_else(|| ret.as_deref().map(rust_type_location))
                .unwrap_or((None, None));
        }
        T::Inferred => None,
    };
    token
        .map(|token| (Some(token.line), Some(token.col)))
        .unwrap_or((None, None))
}

#[flutter_rust_bridge::frb(ignore)]
pub fn check_print(
    diagnostics: &mut Diagnostics,
    expected: &RustStatement,
    actual: &RustStatement,
) -> bool {
    let (Some(a), Some(b)) = (print_value(expected), print_value(actual))
    else {
        let location = statement_location(actual);
        let (line, col) = location.unwrap_or((None, None));
        diagnostics.record_again(
            "Expected a `print!` or `println!` call",
            line,
            col,
        );
        return false;
    };
    compare_expr(diagnostics, a, b)
}

fn print_value(statement: &RustStatement) -> Option<&RustExpression> {
    let RustStatement::Expression {
        expression: RustExpression::FunctionCall { callee, args },
        ..
    } = statement
    else {
        return None;
    };
    let RustExpression::Identifier(name) = callee.as_ref() else { return None };
    if !is_print_name(&name.text) {
        return None;
    }
    match args.as_slice() {
        [value] => Some(value),
        [RustExpression::Literal(format), value, ..]
            if format.text.contains("{}") =>
        {
            Some(value)
        }
        _ => None,
    }
}

#[flutter_rust_bridge::frb(ignore)]
pub fn compare_expr(
    diagnostics: &mut Diagnostics,
    expected: &RustExpression,
    actual: &RustExpression,
) -> bool {
    if expression_matches(expected, actual) {
        return true;
    }
    let (line, col) = expression_location(actual).unwrap_or((None, None));
    diagnostics.record_hard(
        "The expression does not match the drill",
        line,
        col,
    );
    false
}

pub(super) fn statement_location(
    statement: &RustStatement,
) -> Option<(Option<usize>, Option<usize>)> {
    match statement {
        RustStatement::Let { init: Some(expression), .. }
        | RustStatement::Expression { expression, .. }
        | RustStatement::Return(Some(expression)) => {
            expression_location(expression)
        }
        _ => None,
    }
}

fn expression_location(
    expression: &RustExpression,
) -> Option<(Option<usize>, Option<usize>)> {
    use RustExpression as E;
    let token: &Token2 = match expression {
        E::Identifier(token) | E::Literal(token) => token,
        E::Path(tokens) => tokens.first()?,
        E::UnaryOp { op, .. }
        | E::UnaryMutable { op, .. }
        | E::BinaryOp { op, .. } => op,
        E::FieldAccess { field, .. } => field,
        E::FunctionCall { callee, .. } => return expression_location(callee),
        E::Index { receiver, .. } => return expression_location(receiver),
        E::Parenthesized(inner) => return expression_location(inner),
        E::Block(_)
        | E::If { .. }
        | E::While { .. }
        | E::For { .. }
        | E::Loop(_)
        | E::Match(_, _)
        | E::Tuple(_)
        | E::Array(_)
        | E::Struct(_, _) => return None,
    };
    Some((Some(token.line), Some(token.col)))
}

fn expression_matches(a: &RustExpression, b: &RustExpression) -> bool {
    use RustExpression as E;
    match (a, b) {
        (E::Identifier(a), E::Identifier(b))
        | (E::Literal(a), E::Literal(b)) => a.text == b.text,
        (E::Path(a), E::Path(b)) => {
            a.iter().map(|x| &x.text).eq(b.iter().map(|x| &x.text))
        }
        (E::UnaryOp { op: ao, expr: ae }, E::UnaryOp { op: bo, expr: be }) => {
            ao.text == bo.text && expression_matches(ae, be)
        }
        (
            E::BinaryOp { op: ao, left: al, right: ar },
            E::BinaryOp { op: bo, left: bl, right: br },
        ) => {
            ao.text == bo.text
                && ((expression_matches(al, bl) && expression_matches(ar, br))
                    || (matches!(
                        ao.text.as_str(),
                        "+" | "*" | "==" | "!=" | "&&" | "||"
                    ) && expression_matches(al, br)
                        && expression_matches(ar, bl)))
        }
        (
            E::FunctionCall { callee: ac, args: aa },
            E::FunctionCall { callee: bc, args: ba },
        ) => {
            expression_matches(ac, bc)
                && aa.len() == ba.len()
                && aa.iter().zip(ba).all(|(a, b)| expression_matches(a, b))
        }
        (
            E::FieldAccess { receiver: ar, field: af },
            E::FieldAccess { receiver: br, field: bf },
        ) => af.text == bf.text && expression_matches(ar, br),
        (
            E::Index { receiver: ar, index: ai },
            E::Index { receiver: br, index: bi },
        ) => expression_matches(ar, br) && expression_matches(ai, bi),
        (E::Parenthesized(a), b) => expression_matches(a, b),
        (a, E::Parenthesized(b)) => expression_matches(a, b),
        (E::Tuple(a), E::Tuple(b)) | (E::Array(a), E::Array(b)) => {
            a.len() == b.len()
                && a.iter().zip(b).all(|(a, b)| expression_matches(a, b))
        }
        _ => false,
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
    diagnostics.record_hard(
        format!(
            "Expected expression `{}` but found `{}`",
            expected.join(" "),
            actual.join(" ")
        ),
        None,
        None,
    );
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
            None,
            None,
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
        diagnostics.record_hard(
            "Expecting an initialized `let` declaration",
            None,
            None,
        );
        return false;
    };
    let mut result = true;
    let expected_head = &expected[..expected_eq];
    let actual_head = &actual[..actual_eq];
    if expected_head.first().map(String::as_str) != Some("let")
        || actual_head.first().map(String::as_str) != Some("let")
    {
        diagnostics.record_again("Expecting a `let` declaration", None, None);
        return false;
    }
    let expected_mutable =
        expected_head.get(1).map(String::as_str) == Some("mut");
    let actual_mutable = actual_head.get(1).map(String::as_str) == Some("mut");
    if expected_mutable != actual_mutable {
        diagnostics.record_hard(
            "The variable mutability does not match the drill",
            None,
            None,
        );
        result = false;
    }
    let expected_name_index = if expected_mutable { 2 } else { 1 };
    let actual_name_index = if actual_mutable { 2 } else { 1 };
    if expected_head.get(expected_name_index)
        != actual_head.get(actual_name_index)
    {
        diagnostics.record_hard(
            format!(
                "Expected variable `{}` but found `{}`",
                expected_head
                    .get(expected_name_index)
                    .map(String::as_str)
                    .unwrap_or(""),
                actual_head
                    .get(actual_name_index)
                    .map(String::as_str)
                    .unwrap_or("")
            ),
            None,
            None,
        );
        result = false;
    }
    if expected_head.get(expected_name_index + 1..)
        != actual_head.get(actual_name_index + 1..)
    {
        diagnostics.record_hard(
            "The variable type annotation does not match the drill",
            None,
            None,
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
        diagnostics.record_again(
            "Expected a `print!` or `println!` call",
            None,
            None,
        );
        return false;
    };
    let Some(expected_value) = print_token_value(&expected_args) else {
        diagnostics.record_hard(
            "Could not read the expected printed value",
            None,
            None,
        );
        return false;
    };
    let Some(actual_value) = print_token_value(&actual_args) else {
        diagnostics.record_hard("Could not read the printed value", None, None);
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
    if !tokens.first().is_some_and(|name| is_print_name(name))
        || tokens.get(1).map(String::as_str) != Some("!")
        || tokens.get(2).map(String::as_str) != Some("(")
        || matching_token(tokens, 2, "(", ")")? != tokens.len() - 1
    {
        return None;
    }
    Some(split_tokens(&tokens[3..tokens.len() - 1], ","))
}

fn is_print_name(name: &str) -> bool {
    matches!(name, "print" | "println")
}

fn print_token_value(args: &[Vec<String>]) -> Option<&[String]> {
    match args {
        [single] => Some(single),
        [format, value] if format == &["\"{}\"".to_string()] => Some(value),
        _ => None,
    }
}
