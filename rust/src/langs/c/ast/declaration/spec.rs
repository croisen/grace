use crate::core::parser::pratt::Token2;
use crate::langs::c::ast::CParserState;

#[derive(Debug, Clone)]
pub enum CDeclSpec {
    StorageClass(CStorageClassSpec),
    TypeSpec(CTypeSpec),
    TypeQualifier(CTypeQualifier),
    FunctionSpec(CFunctionSpec),
}

impl CDeclSpec {
    pub fn new(state: &mut CParserState<'_>) -> Option<Self> {
        if let Some(storage_class) = CStorageClassSpec::new(state) {
            Some(Self::StorageClass(storage_class))
        } else if let Some(type_spec) = CTypeSpec::new(state) {
            Some(Self::TypeSpec(type_spec))
        } else if let Some(type_qualifier) = CTypeQualifier::new(state) {
            Some(Self::TypeQualifier(type_qualifier))
        } else if let Some(function_spec) = CFunctionSpec::new(state) {
            Some(Self::FunctionSpec(function_spec))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum CSpecQualifier {
    TypeSpec(CTypeSpec),
    TypeQualifier(CTypeQualifier),
}

impl CSpecQualifier {
    pub fn new(state: &mut CParserState<'_>) -> Option<Self> {
        if let Some(type_spec) = CTypeSpec::new(state) {
            Some(Self::TypeSpec(type_spec))
        } else if let Some(type_qualifier) = CTypeQualifier::new(state) {
            Some(Self::TypeQualifier(type_qualifier))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum CStorageClassSpec {
    Typedef,
    Extern,
    Static,
    Auto,
    Register,
}

impl CStorageClassSpec {
    pub fn new(state: &mut CParserState<'_>) -> Option<Self> {
        if state.token_eat_text_sen("typedef") {
            state.advance();
            Some(Self::Typedef)
        } else if state.token_eat_text_sen("extern") {
            state.advance();
            Some(Self::Extern)
        } else if state.token_eat_text_sen("static") {
            state.advance();
            Some(Self::Extern)
        } else if state.token_eat_text_sen("register") {
            state.advance();
            Some(Self::Extern)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum CTypeSpec {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    Complex,
    Imaginary,
    StructUnion(Token2, bool), // true if a union
    TypedefName(Token2),
}

impl CTypeSpec {
    pub fn new(state: &mut CParserState<'_>) -> Option<Self> {
        if state.token_eat_text_sen("void") {
            state.advance();
            Some(Self::Void)
        } else if state.token_eat_text_sen("char") {
            state.advance();
            Some(Self::Char)
        } else if state.token_eat_text_sen("short") {
            state.advance();
            Some(Self::Short)
        } else if state.token_eat_text_sen("int") {
            state.advance();
            Some(Self::Int)
        } else if state.token_eat_text_sen("long") {
            state.advance();
            Some(Self::Long)
        } else if state.token_eat_text_sen("float") {
            state.advance();
            Some(Self::Float)
        } else if state.token_eat_text_sen("double") {
            state.advance();
            Some(Self::Double)
        } else if state.token_eat_text_sen("signed") {
            state.advance();
            Some(Self::Signed)
        } else if state.token_eat_text_sen("unsigned") {
            state.advance();
            Some(Self::Unsigned)
        } else if state.token_eat_text_sen("complex") {
            state.advance();
            Some(Self::Complex)
        } else if state.token_eat_text_sen("imaginary") {
            state.advance();
            Some(Self::Imaginary)
        } else if state.token_eat_text_sen("struct") {
            state.advance();
            let Some(token) = state.token_current().map(|x| x.into()) else {
                return None;
            };

            state.advance();
            Some(Self::StructUnion(token, false))
        } else if state.token_eat_text_sen("union") {
            state.advance();
            let Some(token) = state.token_current().map(|x| x.into()) else {
                return None;
            };

            state.advance();
            Some(Self::StructUnion(token, true))
        } else if state.not_at_end() {
            let type_spec =
                Self::TypedefName(state.token_current().unwrap().into());
            state.advance();
            Some(type_spec)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum CTypeQualifier {
    Const,
    Restrict,
    Volatile,
}

impl CTypeQualifier {
    pub fn new(state: &mut CParserState<'_>) -> Option<Self> {
        if state.token_eat_text_sen("const") {
            state.advance();
            Some(Self::Const)
        } else if state.token_eat_text_sen("restrict") {
            state.advance();
            Some(Self::Restrict)
        } else if state.token_eat_text_sen("volatile") {
            state.advance();
            Some(Self::Volatile)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum CFunctionSpec {
    Inline,
}

impl CFunctionSpec {
    pub fn new(state: &mut CParserState<'_>) -> Option<Self> {
        if state.token_eat_text_sen("inline") {
            state.advance();
            Some(Self::Inline)
        } else {
            None
        }
    }
}
