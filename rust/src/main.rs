mod api;

use api::lexers::java::JavaTokenKind;
use api::lexers::rust::RustTokenKind;
use api::lexers::sql::SQLTokenKind;
use api::lexers::Token;

fn main() {
    let source = r#"
        fn square(i: i64) -> i64 {
            let x = 69420;
        }
    "#;

    let tokens = Token::<RustTokenKind>::tokenize(source);
    println!("{tokens:#?}");

    let source = r#"
        INSERT INTO table VALUES (a, b, c, d, e, f);
    "#;

    let tokens = Token::<SQLTokenKind>::tokenize(source);
    println!("{tokens:#?}");

    let source = r#"
        class A {
            public static void main(String[] args) {
                int x = new int(5);
            }
        }
    "#;

    let tokens = Token::<JavaTokenKind>::tokenize(source);
    println!("{tokens:#?}");
}
