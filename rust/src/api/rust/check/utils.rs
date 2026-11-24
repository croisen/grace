use std::sync::Arc;

use oak_rust::ast::{Function, Item};

use crate::api::diagnostics::Diagnostics;

pub fn generated_get_main(g: &Vec<Item>) -> Arc<Function> {
    generated_get_functions(g)
        .iter()
        .find(|&a| a.name.name == "main")
        .unwrap()
        .clone()
}

pub fn generated_get_functions(g: &Vec<Item>) -> Vec<Arc<Function>> {
    g.iter()
        .filter_map(|a| {
            if let Item::Function(f) = a { Some(f.clone()) } else { None }
        })
        .collect()
}

pub fn user_get_main(
    diagnostics: &mut Diagnostics,
    u: &Vec<Item>,
) -> Option<Arc<Function>> {
    user_get_functions(diagnostics, u)
        .iter()
        .find(|&a| a.name.name == "main")
        .map(|a| a.clone())
}

pub fn user_get_functions(
    diagnostics: &mut Diagnostics,
    u: &Vec<Item>,
) -> Vec<Arc<Function>> {
    let r = u
        .iter()
        .filter_map(|a| {
            if let Item::Function(f) = a { Some(f.clone()) } else { None }
        })
        .collect::<Vec<_>>();

    if r.is_empty() {
        diagnostics.record_again("No function declarations found");
    }

    r
}
