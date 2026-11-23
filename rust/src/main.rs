use crate::api::java::JavaConcept;

pub mod api;

fn main() {
    let drill = JavaConcept::Switch;
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
}
