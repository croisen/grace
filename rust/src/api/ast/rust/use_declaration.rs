use crate::api::GraceError;
use crate::api::ast::rust::item::Item;
use crate::api::ast::rust::paths::SimplePath;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::macros::skip_match;
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[flutter_rust_bridge::frb(ignore)]
pub struct UseDeclaration(UseTree);

impl UseDeclaration {
    pub fn new(
        iter: &mut PrevIter<'_, RustToken>,
        errors: &mut Vec<GraceError>,
    ) -> Self {
        UseDeclaration(UseTree::new(iter, errors))
    }

    pub fn flatten(self) -> Vec<Self> {
        self.0.flatten().into_iter().map(|u| UseDeclaration(u)).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[flutter_rust_bridge::frb(ignore)]
pub struct UseTree {
    main: SimplePath,      // for normal ones
    alias: Option<String>, // for the ones with the `use a as _`
    last: Vec<UseTree>,    // For nested ones
}

impl UseTree {
    pub fn new(
        iter: &mut PrevIter<'_, RustToken>,
        errors: &mut Vec<GraceError>,
    ) -> Self {
        let main = SimplePath::new(iter, errors);
        let mut last = vec![];
        let mut alias = None;

        let token = iter.next();
        if token.is_none() {
            return Self { main, alias, last };
        }

        let token = token.unwrap();
        if token.text == "{" {
            let mut take_more = true;
            while let Some(tk) = iter.next() {
                match tk.kind {
                    RustTokenKind::Identifier => {
                        if !take_more {
                            break;
                        }

                        let _ = iter.prev();
                        last.push(Self::new(iter, errors));
                        // since the recursive new eats the comma too we gotta bring it back
                        let _ = iter.prev();
                        take_more = false;
                    }
                    RustTokenKind::Delimiters => {
                        if tk.text == "," {
                            take_more = true
                        } else {
                            break;
                        }
                    }
                    RustTokenKind::Pairs => break, // last } hopefully
                    _ => {
                        let _ = iter.prev(); // it's a keyword
                        break;
                    }
                }
            }
        } else if token.text == "as" {
            if let Some(token2) = iter.next()
                && token2.kind == RustTokenKind::Identifier
            {
                alias = Some(token2.text.clone());
            }
        }

        skip_match!(iter, RustTokenKind::Delimiters);
        Self { main, alias, last }
    }

    pub fn flatten(self) -> Vec<Self> {
        let mut v = vec![];
        let mut current = vec![];
        Self::flatten_mid(&mut v, &mut current, self);
        v
    }

    fn flatten_mid(
        result: &mut Vec<Self>,
        current: &mut Vec<SimplePath>,
        last: Self,
    ) {
        if last.last.is_empty() {
            let mut main = SimplePath(vec![]);
            for c in current {
                for path in &c.0 {
                    main.0.push(path.clone());
                }
            }

            for path in &last.main.0 {
                main.0.push(path.clone());
            }

            result.push(Self { main, last: vec![], alias: last.alias })
        } else {
            current.push(last.main);
            for l in last.last {
                Self::flatten_mid(result, current, l);
            }

            let _ = current.pop();
        }
    }
}

impl From<UseDeclaration> for Item {
    fn from(value: UseDeclaration) -> Self {
        Self::UseDecl(value)
    }
}
