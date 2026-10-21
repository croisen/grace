use crate::api::GraceError;
use crate::api::ast::rust::RustAst;
use crate::api::ast::rust::item::Item;
use crate::api::ast::rust::visibility::Visibility;
use crate::api::lexers::rust::RustToken;
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone, PartialEq)]
#[flutter_rust_bridge::frb(ignore)]
pub struct VisItem {
    visibility: Option<Visibility>,
    item: Item,
}

impl VisItem {
    pub fn new(
        iter: &mut PrevIter<'_, RustToken>,
        errors: &mut Vec<GraceError>,
    ) -> Vec<Self> {
        let mut v = vec![];
        let visibility = Visibility::new(iter, errors);

        let item = Item::new(iter, errors, visibility.is_some());
        if let Item::UseDecl(item) = item {
            for item in item.flatten() {
                v.push(Self {
                    visibility: visibility.clone(),
                    item: item.into(),
                })
            }
        } else {
            v.push(Self { visibility, item: item.into() })
        }

        v
    }
}

impl From<VisItem> for RustAst {
    fn from(value: VisItem) -> Self {
        Self::Item(value)
    }
}
