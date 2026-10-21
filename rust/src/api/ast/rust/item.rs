use crate::api::GraceError;
use crate::api::ast::rust::use_declaration::UseDeclaration;
use crate::api::lexers::LexingError;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone, PartialEq)]
#[flutter_rust_bridge::frb(ignore)]
pub enum Item {
    // Module,
    // ExternCrate,
    UseDecl(UseDeclaration),
    Function,
    TypeAlias,
    Struct,
    Enum,
    Union,
    ConstantItem,
    StaticItem,
    Trait,
    Implementation,
    // ExternBlock,
    None,
}

impl Item {
    pub fn new(
        iter: &mut PrevIter<'_, RustToken>,
        errors: &mut Vec<GraceError>,
        visibility_found: bool,
    ) -> Self {
        let token = iter.next();
        if token.is_none() {
            return Self::None;
        }

        let token = token.unwrap();
        match token.kind {
            RustTokenKind::StrictKeyword => match token.text.as_str() {
                "use" => Self::UseDecl(UseDeclaration::new(iter, errors)),
                _ => Self::None,
            },
            RustTokenKind::Identifier => {
                if visibility_found {
                    GraceError::syntax_error(
                        errors,
                        token,
                        "expecting an item",
                    );
                }

                Self::None
            }
            _ => Self::None,
        }
    }
}
