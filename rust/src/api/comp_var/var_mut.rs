pub fn var_mut(ifmut: usize) -> (String, String) {
    let mut result = String::new();
    let mut condition = String::new();
    match ifmut {
        1 => { result = " mut".to_string(); condition = " - must be mut.".to_string(); }
        0 => { result = "".to_string(); condition = " - must not be mut.".to_string(); }
        _ => result = "invalid input".to_string(),
    }
    (result, condition)
}
