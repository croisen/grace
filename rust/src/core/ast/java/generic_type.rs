use crate::api::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct GenericType {
    pub base_type: String,
    pub type_arguments: Vec<String>,
}
