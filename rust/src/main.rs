mod comp_var;

fn main() {
    let (variable, condition) = comp_var::var_gen::var_gen(1, 4);
    println!("{}", condition);
    
    println!("\n{}\n", variable);

    let (var_two, cond_two) = comp_var::var_gen::var_gen(0, 5);
    
    println!("{}", cond_two);
    println!("\n{}", var_two);
}
