mod rust_drill_generator;

use crate::rust_drill_generator::generate_drill;

fn main() {
    let (answer, conditions, output) = generate_drill(2);

    println!("===== ANSWER =====");
    println!("{}", answer);

    println!("\n===== CONDITIONS =====");
    println!("{}", conditions);

    println!("\n===== OUTPUT =====");
    println!("{}", output);
}
