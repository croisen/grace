use rand::RngExt;

pub fn indenter(code: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    let mut result = String::new();
    for line in code.lines() {
        if !line.trim().is_empty() {
            result.push_str(&pad);
        }
        result.push_str(line);
        result.push('\n');
    }
    if result.ends_with('\n') {
        result.pop();
    }
    result
}

fn callsign() -> String {
    let mut rng = rand::rng();
    let names = [
        "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india",
        "juliett", "kilo", "lima", "mike", "november", "oscar", "papa", "quebec", "romeo",
        "sierra", "tango", "uniform", "victor", "whiskey", "xray", "yankee", "zulu",
    ];
    names[rng.random_range(0..names.len())].to_string()
}

fn make_i32_var() -> Vec<String> {
    let mut rng = rand::rng();
    let mut v = Vec::new();
    v.push(callsign());
    v.push(rng.random_range(0..15).to_string());
    v
}

pub fn generate_drill(difficulty: u8) -> (String, String, String) {
    let mut rng = rand::rng();

    let diff = if difficulty < 1 {
        1
    } else if difficulty > 3 {
        3
    } else {
        difficulty
    };

    let mut features = Vec::new();
    if diff == 1 {
        features.push(rng.random_range(0..4));
    } else if diff == 2 {
        let first = rng.random_range(0..4);
        let mut second = rng.random_range(0..4);
        while second == first {
            second = rng.random_range(0..4);
        }
        features.push(first);
        features.push(second);
    } else {
        features.push(0);
        features.push(1);
        features.push(2);
        features.push(3);
    }

    let want_vars = features.contains(&0);
    let want_if = features.contains(&1);
    let want_while = features.contains(&2);
    let want_fn = features.contains(&3);

    let mut code_parts: Vec<String> = Vec::new();
    let mut conditions: Vec<String> = Vec::new();
    let mut output = String::new();
    let mut step = 1;

    let mut helper_fn_code = String::new();
    let mut helper_name = String::new();
    if want_fn {
        helper_name = callsign();
        helper_fn_code = format!(
            "fn {}() {{\n    println!(\"Hello from function!\");\n}}\n\n",
            helper_name
        );
        conditions.push(format!(
            "{}. Create a function named {} that takes no arguments and prints \"Hello from function!\".",
            step, helper_name
        ));
        step += 1;
    }

    let mut var1_name = String::new();
    let mut var1_val = 0;
    let mut var2_name = String::new();
    let mut var2_val = 0;

    if want_vars || want_if || want_while {
        let v1 = make_i32_var();
        var1_name = v1[0].clone();
        var1_val = v1[1].parse().unwrap_or(0);

        code_parts.push(format!("let mut {} = {};", var1_name, var1_val));
        conditions.push(format!(
            "{}. Inside the main function, declare a mutable i32 variable named {} and set it to {}.",
            step, var1_name, var1_val
        ));
        step += 1;

        if want_if || rng.random_range(0..2) == 0 {
            let v2 = make_i32_var();
            var2_name = v2[0].clone();
            var2_val = v2[1].parse().unwrap_or(0);
            while var2_name == var1_name {
                let v2 = make_i32_var();
                var2_name = v2[0].clone();
                var2_val = v2[1].parse().unwrap_or(0);
            }
            code_parts.push(format!("let mut {} = {};", var2_name, var2_val));
            conditions.push(format!(
                "{}. Inside the main function, declare a mutable i32 variable named {} and set it to {}.",
                step, var2_name, var2_val
            ));
            step += 1;
        }
    }

    if want_fn {
        code_parts.push(format!("{}();", helper_name));
        conditions.push(format!(
            "{}. Inside the main function, call the function {}.",
            step, helper_name
        ));
        step += 1;
        output.push_str("Hello from function!\n");
    }

    if want_if {
        let make_true = rng.random_range(0..2) == 0;
        let op = if make_true { "==" } else { "!=" };

        if make_true {
            var2_val = var1_val;
        } else {
            if var2_val == var1_val {
                var2_val = var1_val + 1;
            }
        }

        if !var2_name.is_empty() {
            for i in 0..code_parts.len() {
                if code_parts[i].contains(&var2_name) && code_parts[i].starts_with("let mut") {
                    code_parts[i] = format!("let mut {} = {};", var2_name, var2_val);
                    break;
                }
            }
        }

        let condition_str = format!("{} {} {}", var1_name, op, var2_name);

        let mut if_body = String::new();
        let mut else_body = String::new();

        if want_while {
            let (while_code, while_out, _) = make_while(&var1_name, &mut step);
            if_body = while_code;
            conditions.push(format!(
                "{}. Inside the main function, write an if statement that checks whether {}. Inside the if, put the while loop described next.",
                step - 1, condition_str
            ));
            if make_true {
                output.push_str(&while_out);
            } else {
                else_body = "println!(\"else branch\");".to_string();
                output.push_str("else branch\n");
            }
        } else {
            if_body = "println!(\"if is true\");".to_string();
            else_body = "println!(\"else branch\");".to_string();
            if make_true {
                output.push_str("if is true\n");
            } else {
                output.push_str("else branch\n");
            }
            conditions.push(format!(
                "{}. Inside the main function, write an if-else statement that checks whether {}. If true print \"if is true\", otherwise print \"else branch\".",
                step, condition_str
            ));
            step += 1;
        }

        let if_code = format!(
            "if {} {{\n{}\n}} else {{\n{}\n}}",
            condition_str,
            indenter(&if_body, 4),
            indenter(&else_body, 4)
        );
        code_parts.push(if_code);
    }

    if want_while && !want_if {
        let (while_code, while_out, _) = make_while(&var1_name, &mut step);
        code_parts.push(while_code);
        output.push_str(&while_out);
    }

    let main_body = code_parts.join("\n");
    let full_code = format!(
        "{}fn main() {{\n{}\n}}",
        helper_fn_code,
        indenter(&main_body, 4)
    );

    let conditions_str = conditions.join("\n");

    if output.ends_with('\n') {
        output.pop();
    }

    (full_code, conditions_str, output)
}

fn make_while(counter_name: &str, step: &mut i32) -> (String, String, i32) {
    let mut rng = rand::rng();
    let limit = rng.random_range(2..5);

    let body = format!(
        "println!(\"loop\");\n{} = {} + 1;",
        counter_name, counter_name
    );

    let code = format!(
        "while {} < {} {{\n{}\n}}",
        counter_name,
        limit,
        indenter(&body, 4)
    );

    let mut out = String::new();
    for _ in 0..limit {
        out.push_str("loop\n");
    }

    *step += 1;

    (code, out, *step)
}
