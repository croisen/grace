use crate::api::rust::RustConcept;

pub mod api;

fn main() {
    let drill = RustConcept::VarDecl;
    let mut d = drill.make_drill();
    let diag = d.check(d.answer.clone());

    println!("Diagnostics: {:?}", diag.grade());
    for d in diag {
        println!("\t{}", d.to_string());
    }

    println!("Conditions");
    for c in d.conditions {
        println!("{:>5} {}", c.0, c.1);
    }

    println!("{}", d.answer);
}
