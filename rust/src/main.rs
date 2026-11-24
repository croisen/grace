pub mod api;
pub mod core;
pub mod langs;
pub mod utils;

use crate::api::java::JavaConcept;
use crate::api::rust::RustConcept;
use crate::api::sql::SqlConcept;

fn main() {
    run_rust_drills();
    run_java_drills();
    run_sql_drills();
}

fn run_rust_drills() {
    let concepts = [
        RustConcept::VarDecl,
        RustConcept::FunctionDecl,
        RustConcept::Ifs,
        RustConcept::ForLoop,
        RustConcept::WhileLoop,
        RustConcept::Loop,
        RustConcept::Match,
    ];

    for concept in concepts {
        let mut drill = concept.make_drill();
        let faulty_code = faulty_rust_answer(concept, &drill.answer);
        let diagnostics = drill.check(faulty_code.clone());

        println!("\nRust {concept:?}: {:?}", diagnostics.grade());
        println!("Code:\n{faulty_code}");
        for (passed, condition) in &drill.conditions {
            println!("  [{}] {condition}", if *passed { "x" } else { " " });
        }
        for diagnostic in diagnostics.iter() {
            println!("  {} ({:?})", diagnostic.to_string(), diagnostic.grade);
        }
    }
}

fn run_java_drills() {
    let concepts = [
        JavaConcept::VarDecl,
        JavaConcept::FunctionDecl,
        JavaConcept::Ifs,
        JavaConcept::ForLoop,
        JavaConcept::WhileLoop,
        JavaConcept::DoWhile,
        JavaConcept::Switch,
    ];

    for concept in concepts {
        let mut drill = concept.make_drill();
        let faulty_code = faulty_java_answer(concept, &drill.answer);
        let diagnostics = drill.check(faulty_code.clone());

        println!("\nJava {concept:?}: {:?}", diagnostics.grade());
        println!("Code:\n{faulty_code}");
        for (passed, condition) in &drill.conditions {
            println!("  [{}] {condition}", if *passed { "x" } else { " " });
        }
        for diagnostic in diagnostics.iter() {
            println!("  {} ({:?})", diagnostic.to_string(), diagnostic.grade);
        }
        if matches!(concept, JavaConcept::WhileLoop | JavaConcept::DoWhile) {
            let missing_update = without_loop_update(&drill.answer);
            let missing_diagnostics = drill.check(missing_update);
            println!(
                "Java {concept:?} without update: {:?}",
                missing_diagnostics.grade()
            );
            for diagnostic in missing_diagnostics.iter() {
                println!(
                    "  {} ({:?})",
                    diagnostic.to_string(),
                    diagnostic.grade
                );
            }
        }
    }
}

fn run_sql_drills() {
    let concepts = [
        SqlConcept::Select,
        SqlConcept::Where,
        SqlConcept::OrderBy,
        SqlConcept::GroupBy,
        SqlConcept::Having,
        SqlConcept::Join,
        SqlConcept::Aggregate,
        SqlConcept::Insert,
        SqlConcept::Update,
        SqlConcept::Delete,
        SqlConcept::CreateTable,
        SqlConcept::AlterTable,
        SqlConcept::DropTable,
        SqlConcept::TruncateTable,
        SqlConcept::AddColumn,
        SqlConcept::DropColumn,
        SqlConcept::RenameTable,
    ];

    for concept in concepts {
        let mut drill = concept.make_drill();
        let faulty_sql = faulty_answer(concept, &drill.answer);
        let diagnostics = drill.check(faulty_sql.clone());

        println!("\n{concept:?}: {:?}", diagnostics.grade());
        println!("SQL: {faulty_sql}");
        for (passed, condition) in &drill.conditions {
            println!("  [{}] {condition}", if *passed { "x" } else { " " });
        }
        for diagnostic in diagnostics.iter() {
            println!("  {} ({:?})", diagnostic.to_string(), diagnostic.grade);
        }
    }
}

fn faulty_rust_answer(concept: RustConcept, answer: &str) -> String {
    match concept {
        RustConcept::VarDecl => replace_first_integer(answer, "999"),
        RustConcept::FunctionDecl => answer.replacen("a + b", "a - b", 1),
        RustConcept::Ifs => answer.replacen("\"pass\"", "\"wrong\"", 1),
        RustConcept::ForLoop => replace_after(answer, "..", "999", '{'),
        RustConcept::WhileLoop => answer.replacen("+= 1", "+= 2", 1),
        RustConcept::Loop => answer.replacen("Hello, Rust!", "Wrong", 1),
        RustConcept::Match => answer.replacen("2 =>", "9 =>", 1),
    }
    .replace("println!", "print!")
}

fn faulty_java_answer(concept: JavaConcept, answer: &str) -> String {
    match concept {
        JavaConcept::VarDecl => replace_after(answer, " = ", "0", ';'),
        JavaConcept::FunctionDecl => answer.replacen("a + b", "a - b", 1),
        JavaConcept::Ifs => answer.replacen("\"pass\"", "\"wrong\"", 1),
        JavaConcept::ForLoop
        | JavaConcept::WhileLoop
        | JavaConcept::DoWhile => flip_update_operator(answer),
        JavaConcept::Switch => answer.replacen("case 2:", "case 9:", 1),
    }
    .replace("System.out.println", "System.out.print")
}

fn flip_update_operator(answer: &str) -> String {
    let changed = answer.replacen("++", "--", 1);
    if changed == answer { answer.replacen("--", "++", 1) } else { changed }
}

fn without_loop_update(answer: &str) -> String {
    answer
        .lines()
        .filter(|line| !line.contains("++") && !line.contains("--"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn replace_first_integer(input: &str, replacement: &str) -> String {
    let Some(start) = input.find(|c: char| c.is_ascii_digit()) else {
        return input.to_owned();
    };
    let end = start
        + input[start..]
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(input.len() - start);
    format!("{}{}{}", &input[..start], replacement, &input[end..])
}

fn replace_after(
    input: &str,
    marker: &str,
    replacement: &str,
    delimiter: char,
) -> String {
    let Some(start) = input.find(marker).map(|i| i + marker.len()) else {
        return input.to_owned();
    };
    let end = input[start..]
        .find(delimiter)
        .map(|i| start + i)
        .unwrap_or(input.len());
    format!("{}{}{}", &input[..start], replacement, &input[end..])
}

fn faulty_answer(concept: SqlConcept, answer: &str) -> String {
    match concept {
        SqlConcept::Select => {
            replace_before(answer, "SELECT ", "wrong_column", '\n')
        }
        SqlConcept::Where => {
            replace_before(answer, "WHERE ", "wrong_column = 0", ';')
        }
        SqlConcept::OrderBy => {
            replace_before(answer, "ORDER BY ", "wrong_column ASC", ';')
        }
        SqlConcept::GroupBy => {
            replace_before(answer, "GROUP BY ", "wrong_column", ';')
        }
        SqlConcept::Having => {
            replace_before(answer, "HAVING ", "SUM(grade) > 0", ';')
        }
        SqlConcept::Join => {
            replace_before(answer, "INNER JOIN ", "wrong_table", '\n')
        }
        SqlConcept::Aggregate => {
            replace_before(answer, " AS ", " wrong_alias", '\n')
        }
        SqlConcept::Insert => {
            replace_before(answer, "VALUES (", " 'wrong', 0)", ';')
        }
        SqlConcept::Update => replace_before(answer, "SET ", "age = 0", '\n'),
        SqlConcept::Delete => answer.replacen("students", "wrong_table", 1),
        SqlConcept::CreateTable => answer.replacen("id INT", "id TEXT", 1),
        SqlConcept::AlterTable
        | SqlConcept::AddColumn
        | SqlConcept::DropColumn => {
            answer.replacen("employees", "wrong_table", 1)
        }
        SqlConcept::DropTable => {
            answer.replacen("temporary_employees", "wrong_table", 1)
        }
        SqlConcept::TruncateTable => answer.replacen("logs", "wrong_table", 1),
        SqlConcept::RenameTable => {
            answer.replacen("RENAME TO employees", "RENAME TO wrong_table", 1)
        }
    }
}

fn replace_before(
    answer: &str,
    marker: &str,
    replacement: &str,
    delimiter: char,
) -> String {
    let Some(start) = answer.find(marker).map(|index| index + marker.len())
    else {
        return answer.to_owned();
    };
    let end = answer[start..]
        .find(delimiter)
        .map(|offset| start + offset)
        .unwrap_or(answer.len());
    format!("{}{}{}", &answer[..start], replacement, &answer[end..])
}
