use crate::api::GraceError;
use crate::api::lexers::rust::RustToken;
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone, PartialEq)]
#[flutter_rust_bridge::frb(ignore)] // <- Here
pub enum Safety {
    Safe,
    Unsafe,
}

impl Safety {
    // Why are you suddenly not ignoring the impl of an ignored data type
    #[flutter_rust_bridge::frb(ignore)]
    pub fn new(
        iter: &mut PrevIter<'_, RustToken>,
        errors: &mut Vec<GraceError>,
    ) -> Option<Self> {
        let s = if let Some(token) = iter.next() {
            if token.text == "unsafe" {
                Some(Self::Unsafe)
            } else {
                let _ = iter.prev();
                Some(Self::Safe)
            }
        } else {
            let p = iter.prev().unwrap();
            GraceError::syntax_error(errors, p, "Unexpected EOF");
            let _ = iter.next();
            None
        };

        s
    }
}
