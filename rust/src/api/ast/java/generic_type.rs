use crate::api::GraceError;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};
use crate::utils::prev_iter::PrevIter;

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub struct GenericType {
    pub base_type: String,
    pub type_arguments: Vec<String>,
}
