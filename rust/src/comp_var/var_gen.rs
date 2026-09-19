use super::{var_mut, var_name, var_type};

pub fn var_gen(input_a: usize, input_b: usize) -> (String, String) {
    let _var_gen = Vec::<String>::new();
    let (is_mut, condition_mut) = var_mut::var_mut(input_a);
    let (name, condition_name) = var_name::var_name();
    let (type_name, value, condition_type, condition_value) = var_type::var_type(input_b);
    let variable = format!("let{is_mut} {name}: {type_name} = {value};");
    let conditions = format!("{condition_mut}\n{condition_name}\n{condition_type}\n{condition_value}"
                            );
    (variable, conditions)
}
