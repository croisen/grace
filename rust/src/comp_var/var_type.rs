use rand::prelude::*;
use super::var_name;

pub fn var_type(input: usize) -> (String, String, String, String) {
    let mut rng = rand::rng();
    let mut var_value = String::new();
    let var_types = ["u8", "u32", "u64", "i8", "i32", "i64", "bool", "&str"];
    let var_type = var_types[input].to_string();
    let (string_val, _) = var_name::var_name();
    match input {
        0 => var_value = rng.random_range(0..=255).to_string(),
        1 => var_value = rng.random_range(0..=500).to_string(),
        2 => var_value = rng.random_range(0..=500).to_string(),
        3 => var_value = rng.random_range(-128..=127).to_string(),
        4 => var_value = rng.random_range(-500..=500).to_string(),
        5 => var_value = rng.random_range(-500..=500).to_string(),
        6 => var_value = rng.random_bool(0.50).to_string(),
        7 => var_value = string_val,
        _ => var_value = "error".to_string(),
    }
    let var_condition_type = format!(" - must be variable type: {}", var_type);
    let var_condition_value = format!(" - must have value: {}", var_value);
    (var_type, var_value, var_condition_type, var_condition_value)
}
