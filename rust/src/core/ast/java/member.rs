use crate::api::ast::java::class::JavaClass;
use crate::api::ast::java::constructor::JavaConstructor;
use crate::api::ast::java::field::JavaField;
use crate::api::ast::java::method::JavaMethod;
use crate::api::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore)]
pub(crate) enum JavaMember {
    Method(JavaMethod),
    Field(JavaField),
    Constructor(JavaConstructor),
    InnerClass(JavaClass),
}
