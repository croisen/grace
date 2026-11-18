// #![allow(dead_code, unused)] // remove later
mod api;
mod core;
mod langs;
mod utils;

use crate::langs::c::ast::CAst;

fn main() {
    let source = r#"
        void main(int argc, char **argv) {
            return;
        }
    "#;

    let c = CAst::new(source);
}
