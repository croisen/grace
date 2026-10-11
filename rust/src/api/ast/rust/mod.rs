use crate::api::ast::AstOperation;
use crate::api::lexers::GraceError;
use crate::api::lexers::rust::{RustToken, RustTokenKind};
use crate::utils::macros::any;
use crate::utils::prev_iter::PrevIter;

mod imports;
mod variables;

#[derive(Default, Debug, Clone, PartialEq)]
// More like IR but sure
pub enum RustAst {
    Import {
        pub_: bool,
        main: Vec<String>,   // std::path:: ...
        final_: Vec<String>, // ... ::{Path, PathBuf, ...} or ... ::Path
    },

    VariableDecl {
        static_: bool,
        const_: bool,
        pub_: bool,
        name: String,
        type_: Option<String>,
        value: Box<RustAst>,
    },

    FunctionDecl {
        pub_: bool,
        mut_: bool,
        name: String,
        params: Vec<(String, Option<String>)>, // name, optional type (because self)
        return_: String,
    },
    FunctionImpl {
        pub_: bool,
        name: String,
        params: Vec<(String, Option<String>)>, // name, optional type (because self)
        return_: String,
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
        enumvalue_: Vec<String>, // just named values
        tupletagged_: Vec<(String, Vec<String>)>, // sum types Some(type, ...)
        structtagged_: Vec<(String, Vec<(String, String)>)>, // like RustAst
    },
    RawEnum {
        name: String,
        children: Vec<String>,
    },

    ZeImpl {
        name: String,           // impl RustAst
        for_: Option<String>,   // impl Default for RustAst
        children: Vec<RustAst>, // function declarations
    },

    IfConditional {
        condition: Vec<String>,
        children: Vec<RustAst>,
        else_: Box<RustAst>,
    },
    IfLet {
        variant: String,
        variable: String,
        enum_: String,
        children: Vec<RustAst>,
        else_: Box<RustAst>,
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
        enum_: String,
        children: Vec<RustAst>,
    },

    TraitDecl {
        pub_: bool,
        declarations: Vec<RustAst>,
    },

    ActualAST {
        operation: AstOperation,
        text: String,
        left: Box<RustAst>,
        right: Box<RustAst>,
    },

    #[default]
    Unknown,
}

impl RustAst {
    pub fn new(
        tokens: Vec<RustToken>,
        mut errors: Vec<GraceError>,
    ) -> (Vec<Self>, Vec<GraceError>) {
        let mut ast = vec![];
        let mut iter: PrevIter<'_, RustToken> = tokens.into();
        let (mut pub_, mut static_, mut const_) = (false, false, false);
        while let Some(token) = iter.next() {
            let mut skip_ = false;
            match token.kind {
                RustTokenKind::Pub => {
                    pub_ = true;
                    skip_ = true;
                }
                RustTokenKind::Static => {
                    static_ = true;
                    skip_ = true;
                }
                RustTokenKind::Const => {
                    const_ = true;
                    skip_ = true;
                }
                RustTokenKind::Use => {
                    ast.push(imports::imports(&mut iter, pub_, &mut errors))
                }
                RustTokenKind::Whitespace => {
                    skip_ = any!(pub_, static_, const_);
                }
                RustTokenKind::Let => {}
                _ => {}
            }

            if !skip_ {
                pub_ = false;
                static_ = false;
                const_ = false;
            }
        }

        (ast, errors)
    }
}
