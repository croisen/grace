use rand::RngExt;

fn main() {
   println!("{}", callsign(1));
   let (var, var_conditions) = generate_variable(1, false);
   println!("{}\n{}", var, var_conditions);
}
fn callsign(choice: usize) -> String {
   let callsign_dict = ["alpha", "bravo", "charlie", "delta"];
   let output = callsign_dict[choice].to_string();
   output
}
fn generate_variable(choice: usize, if_mut: bool) -> (String, String) {
   let variable_types = ["String", "i32"];
   let rng = rand::rng().random_range(0..=3);
   let name = callsign(rng);
   let is_mut;
   let mut_state;
   let value;
   let var_type = variable_types[choice];
   match choice {
   	 0 => value = callsign(rng),
     1 => value = rand::rng().random_range(0..=257).to_string(),
     _ => value = "error".to_string()
   };
   match if_mut {
        true => { is_mut = "- variable {} must be mutable.".to_string(); mut_state = " mut"; }
        false => { is_mut = "- variable {} must not be mutable.".to_string(); mut_state = ""; }
       // _ => { is_mut = "- variable could either be mutable or not.".to_string(); mut_state = "" }
   }
   (format!("let{} {} {} = {};", mut_state, name, var_type, value),
   format!("- variable must be named \"{}\"\n- variable \"{}\" must be of type \"{}\"\n- variable \"{}\" must have a value of \"{}\"\n{}", 
       name, name, var_type, name, value, is_mut))
}
