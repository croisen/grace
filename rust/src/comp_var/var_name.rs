use rand::RngExt;

use rand::prelude::*;

pub fn var_name() -> (String, String) {
    let mut rng = rand::rng();

    let names = ["alpha", "bravo", "charlie", "delta"];
    let rindex = rng.random_range(0..names.len());
    let rng_name = names[rindex];

    let condition = format!(" - must be named: {}", rng_name);

    (rng_name.to_string(), condition)
}
