use crate::api::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) struct JavaTypeParameter {
    pub name: String,
    pub constraints: Vec<String>,
}
