use crate::api::ast::rust::RustAst;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::api::lexers::{GraceError, LexingError};
use crate::utils::prev_iter::PrevIter;

#[flutter_rust_bridge::frb(ignore)]
pub fn imports(
    iter: &mut PrevIter<'_, RustToken>,
    pub_: bool,
    errors: &mut Vec<GraceError>,
) -> RustAst {
    let mut main = vec![];
    let mut final_ = vec![];
    let mut prev = RustTokenKind::Whitespace;
    let mut is_final = false;
    let _ = iter.next(); // it's a space in use" "std::...
    while let Some(token) = iter.next() {
        match token.kind {
            RustTokenKind::Identifier => {
                if is_final {
                    final_.push(token.text.to_string());
                } else {
                    main.push(token.text.to_string());
                }

                prev = RustTokenKind::Identifier;
            }
            RustTokenKind::DColon => {
                if prev == RustTokenKind::DColon {
                    errors.push(GraceError::new(
                        LexingError::SyntaxError,
                        token.text.to_owned(),
                        "expected identifier".to_owned(),
                        token.line,
                        token.col,
                    ));

                    let _ = iter.skip_while(|t| {
                        !matches!(t.kind, RustTokenKind::Newline)
                    });
                    break;
                }

                prev = RustTokenKind::DColon;
            }
            RustTokenKind::Colon => {
                errors.push(GraceError::new(
                    LexingError::SyntaxError,
                    token.text.to_owned(),
                    "expected `::` found `:`".to_owned(),
                    token.line,
                    token.col,
                ));

                let _ = iter
                    .skip_while(|t| !matches!(t.kind, RustTokenKind::Newline));
                break;
            }
            RustTokenKind::Whitespace => {
                if !is_final {
                    errors.push(GraceError::new(
                        LexingError::SyntaxError,
                        token.text.to_owned(),
                        format!("expected `;` found `{}`", token.text),
                        token.line,
                        token.col,
                    ));

                    let _ = iter.skip_while(|t| {
                        !matches!(t.kind, RustTokenKind::Newline)
                    });
                    break;
                }
            }
            RustTokenKind::OCurly => is_final = true,
            RustTokenKind::CCurly => {}
            RustTokenKind::Semicolon => prev = RustTokenKind::Semicolon,
            RustTokenKind::Comma => {
                if !is_final {
                    if prev == RustTokenKind::DColon {
                        errors.push(GraceError::new(
                            LexingError::SyntaxError,
                            token.text.to_owned(),
                            "expected identifier".to_owned(),
                            token.line,
                            token.col,
                        ));
                    } else {
                        errors.push(GraceError::new(
                            LexingError::SyntaxError,
                            token.text.to_owned(),
                            "expected `;`".to_owned(),
                            token.line,
                            token.col,
                        ));
                    }

                    let _ = iter.skip_while(|t| {
                        !matches!(t.kind, RustTokenKind::Newline)
                    });

                    break;
                }
            }
            RustTokenKind::Newline => {
                if prev != RustTokenKind::Semicolon {
                    errors.push(GraceError::new(
                        LexingError::SyntaxError,
                        token.text.to_owned(),
                        "expected `;`".to_owned(),
                        token.line,
                        token.col,
                    ));
                }

                break;
            }
            _ => {
                errors.push(GraceError::new(
                    LexingError::SyntaxError,
                    token.text.to_owned(),
                    format!("expected `;` found `{}`", token.text),
                    token.line,
                    token.col,
                ));
                let _ = iter.prev();
                break;
            }
        }
    }

    if final_.is_empty() {
        if let Some(p) = main.pop() {
            final_.push(p);
        }
    }

    RustAst::Import { pub_, main, final_ }
}
