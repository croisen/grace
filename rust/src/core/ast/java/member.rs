use crate::core::ast::java::class::JavaClass;
use crate::core::ast::java::constructor::JavaConstructor;
use crate::core::ast::java::field::JavaField;
use crate::core::ast::java::method::JavaMethod;
use crate::core::lexers::java::{JavaToken, JavaTokenKind};

#[derive(Debug, Clone)]
pub enum JavaMember {
    Method(JavaMethod),
    Field(JavaField),
    Constructor(JavaConstructor),
    InnerClass(JavaClass),
}
