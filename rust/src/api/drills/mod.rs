use rand::random_range;

pub mod java;
pub mod rust;
pub mod sql;

// there are two to prevent the same name to be put out
fn generic_varname1() -> &'static str {
    static VARS1: &[&str] = &[
        "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf",
        "hotel", "india", "juliett", "kilo", "lima", "mike",
    ];

    VARS1[random_range(0..VARS1.len())]
}

fn generic_varname2() -> &'static str {
    static VARS2: &[&str] = &[
        "november", "oscar", "papa", "quebec", "romeo", "sierra", "tango",
        "uniform", "victor", "whiskey", "xray", "yankee", "zulu",
    ];
    VARS2[random_range(0..VARS2.len())]
}

fn generic_operator() -> &'static str {
    static OPS: &[&str] = &["<", "<=", ">=", ">", "==", "!="];
    OPS[random_range(0..OPS.len())]
}
