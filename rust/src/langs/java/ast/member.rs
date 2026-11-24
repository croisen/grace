use crate::langs::java::ast::class::JavaClass;
use crate::langs::java::ast::constructor::JavaConstructor;
use crate::langs::java::ast::field::JavaField;
use crate::langs::java::ast::method::JavaMethod;

#[derive(Debug, Clone)]
pub enum JavaMember {
    Method(JavaMethod),
    Field(JavaField),
    Constructor(JavaConstructor),
    InnerClass(JavaClass),
}
