use crate::api::lexers::rust::RustToken;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum RustAst {
    Import {
        main: Vec<String>,   // std::path:: ...
        _final: Vec<String>, // ... ::{Path, PathBuf, ...} or ... ::Path
    },

    VariableDecl {
        _static: bool,
        _const: bool,
        _pub: bool,
        name: String,
        _type: Option<String>,
        value: Vec<RustAst>,
    },

    FunctionDecl {
        _pub: bool,
        name: String,
        params: Vec<(String, Option<String>)>, // name, optional type (because self)
        _return: String,
    },
    FunctionImpl {
        _pub: bool,
        name: String,
        params: Vec<(String, Option<String>)>, // name, optional type (because self)
        _return: String,
        children: Vec<RustAst>,
    },
    FunctionCall {
        name: String,
        params: Vec<String>,
    },

    StructDecl {
        name: String,
        chidlren: Vec<(String, String)>,
    },
    RawStruct {
        name: String,
        chidlren: Vec<(String, Option<String>)>, // optional because members that have the same name
                                                 // as the variable used here can be shortcutted
    },

    EnumDecl {
        enum_value: Vec<String>, // just named values
        tuple_tagged: Vec<(String, Vec<String>)>, // sum types Some(type, ...)
        struct_tagged: Vec<(String, Vec<(String, String)>)>, // like RustAst
    },
    RawEnum {
        name: String,
        tuple_tagged: Option<Vec<String>>,
        struct_tagged: Option<Vec<(String, Option<String>)>>,
    },

    ZeImpl {
        name: String,           // impl RustAst
        _for: Option<String>,   // impl Default for RustAst
        children: Vec<RustAst>, // function declarations
    },

    IfConditional {
        condition: Vec<String>,
        children: Vec<RustAst>,
        _else: Box<RustAst>,
    },
    IfLet {
        variant: String,
        variable: String,
        _enum: String,
        children: Vec<RustAst>,
        _else: Box<RustAst>,
    },

    ForLoop {
        variables: Vec<String>,
        source: Vec<String>,
    },
    WhileLoop {
        condition: Vec<String>,
        children: Vec<RustAst>,
    },
    WhileLet {
        variant: String,
        variable: String,
        _enum: String,
        children: Vec<RustAst>,
    },

    TraitDecl {
        _pub: bool,
        declarations: Vec<RustAst>,
    },

    #[default]
    Unknown,
}

impl RustAst {
    pub fn new(tokens: Vec<RustToken>) -> Vec<Self> {
        vec![]
    }
}
