use crate::api::GraceError;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaPackage {
    pub name: String,
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct JavaImport {
    pub path: String,
    pub is_static: bool,
}
