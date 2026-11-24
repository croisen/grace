use crate::langs::java::ast::expression::JavaExpression;

#[derive(Debug, Clone)]
pub struct JavaAnnotation {
    pub name: Token2,
    pub arguments: Vec<JavaExpression>,
}

#[derive(Debug, Clone)]
pub struct JavaAnnotationType {
    pub name: Token2,
    pub modifiers: Vec<Token2>,
    pub annotations: Vec<JavaAnnotation>,
    pub elements: Vec<JavaAnnotationElement>,
}

#[derive(Debug, Clone)]
pub struct JavaAnnotationElement {
    pub name: Token2,
    pub r#type: Token2,
    pub default_value: Option<JavaExpression>,
}
use crate::core::parser::pratt::Token2;
