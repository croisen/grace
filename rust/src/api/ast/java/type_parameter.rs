use crate::api::GraceError;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct JavaTypeParameter {
    pub name: String,
    pub constraints: Vec<String>,
}
