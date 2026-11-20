use crate::api::java::{java_for_loop, java_while_loop};

mod api;

fn main() {
    let drill = java_for_loop();
    println!("{}", drill.answer);
    let drill = java_while_loop();
    println!("{}", drill.answer);
}
