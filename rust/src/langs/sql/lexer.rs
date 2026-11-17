use logos::{Lexer, Logos, skip};

use crate::core::errors::kind::GraceErrorKind;
use crate::core::lexers::{LexerExtras, Token};

pub type SQLToken<'a> = Token<'a, SQLTokenKind>;

#[derive(Default, Debug, Clone, PartialEq, Logos)]
#[logos(error = GraceErrorKind, extras = LexerExtras)]
#[rustfmt::skip]
pub enum SQLTokenKind {
    #[regex("(?i:ADD)")]        // constraint
    #[regex("(?i:ALL)")]
    #[regex("(?i:ALTER)")]      // {column, table, view}
    #[regex("(?i:AND)")]
    #[regex("(?i:ANY)")]
    #[regex("(?i:AS)")]
    #[regex("(?i:ASC)")]
    #[regex("(?i:BACKUP)")]     // database (we don't use this in the ast)
    #[regex("(?i:BETWEEN)")]
    #[regex("(?i:BY)")]
    #[regex("(?i:CASE)")]
    #[regex("(?i:CHECK)")]
    #[regex("(?i:COLUMN)")]
    #[regex("(?i:CONSTRAINT)")] // create constraint, (we don't use this in the ast)
    #[regex("(?i:CREATE)")]     // {database, index, table, or replace view, unique index, view, procedure}
    #[regex("(?i:DATABASE)")]
    #[regex("(?i:DEFAULT)")]
    #[regex("(?i:DELETE)")]
    #[regex("(?i:DESC)")]
    #[regex("(?i:DISTINCT)")]
    #[regex("(?i:DROP)")]       // {column, constraint, database, default, index, table, view}
    #[regex("(?i:EXEC)")]
    #[regex("(?i:EXISTS)")]
    #[regex("(?i:FOREIGN)")]    // key
    #[regex("(?i:FROM)")]
    #[regex("(?i:FULL)")]       // outer join
    #[regex("(?i:GROUP)")]      // by
    #[regex("(?i:HAVING)")]
    #[regex("(?i:IF)")]         // if (not)? exists
    #[regex("(?i:IN)")]
    #[regex("(?i:INDEX)")]
    #[regex("(?i:INNER)")]      // join
    #[regex("(?i:INSERT)")]     // into (select)?
    #[regex("(?i:INTO)")]
    #[regex("(?i:IS)")]         // (not)? null
    #[regex("(?i:JOIN)")]
    #[regex("(?i:KEY)")]
    #[regex("(?i:LEFT)")]       // join
    #[regex("(?i:LIKE)")]
    #[regex("(?i:LIMIT)")]
    #[regex("(?i:NOT)")]
    #[regex("(?i:NULL)")]
    #[regex("(?i:OR)")]
    #[regex("(?i:ORDER)")]      // by
    #[regex("(?i:OUTER)")]
    #[regex("(?i:PRIMARY)")]    // key
    #[regex("(?i:PROCEDURE)")]
    #[regex("(?i:REPLACE)")]
    #[regex("(?i:RIGHT)")]      // join
    #[regex("(?i:ROWNUM)")]
    #[regex("(?i:SELECT)")]     // {distinct, into, top}
    #[regex("(?i:SET)")]
    #[regex("(?i:TABLE)")]
    #[regex("(?i:TOP)")]
    #[regex("(?i:TRUNCATE)")]   // table
    #[regex("(?i:UNION)")]      // (all)?
    #[regex("(?i:UNIQUE)")]
    #[regex("(?i:UPDATE)")]
    #[regex("(?i:VALUES)")]
    #[regex("(?i:VIEW)")]
    #[regex("(?i:WHERE)")]
    StrictKeyword,

    ReservedKeyword,

    #[token("+")]
    #[token("-")]
    #[token("*")]
    #[token("/")]
    #[token("<<")]
    #[token(">>")]
    #[token("<")]
    #[token("<=")]
    #[token(">=")]
    #[token(">")]
    #[token("==")]
    #[token("!=")]
    #[token("&")]
    #[token("|")]
    #[token("^")]
    #[token("&&")]
    #[token("||")]
    #[token("+=")]
    #[token("-=")]
    #[token("*=")]
    #[token("/=")]
    #[token("%=")]
    #[token("<<=")]
    #[token("=>>")]
    #[regex("=")]
    Operators,

    #[token("(")]
    #[token(")")]
    #[token("{")]
    #[token("}")]
    #[token("[")]
    #[token("]")]
    Pairs,

    #[token(";")]
    #[token(",")]
    Delimiters,

    #[token(":")]
    Colon,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    #[default]
    Identifier,

    #[regex("[+-]?[0-9][_0-9]*")]
    #[regex("[+-]?0[bB]_*[01][_01]*")]
    #[regex("[+-]?0[oO]_*[0-7][_0-7]*")]
    #[regex("[+-]?0[xX]_*[0-9a-fA-F][_0-9a-fA-F]*")]
    Integer,

    #[regex(r#"[+-]?(([0-9][_0-9]*\.([0-9][_0-9]*)?([eE][+-]?([0-9][_0-9]*))?[fFdD]?)|(\.([0-9][_0-9]*)([eE][+-]?([0-9][_0-9]*))?[fFdD]?)|(([0-9][_0-9]*)([eE][+-]?([0-9][_0-9]*))[fFdD]?)|(([0-9][_0-9]*)([eE][+-]?([0-9][_0-9]*))?[fFdD]))"#)]
    Float,

    #[regex(r#""([^"\\]|\.)*""#, not_string)]
    #[regex(r#"'([^"\\]|\.)*'"#)]
    String,

    #[regex(r"//[^\n]*", allow_greedy = true)]
    #[regex(r"/\*(?:[^*]\*[^/])*\*/")]
    Comment,

    #[regex(r"\n|\r\n")]
    Newline,
    #[regex(r"[ \t]+", skip)]
    Whitespace,
}

fn not_string<'a, T>(lex: &mut Lexer<'a, T>) -> Result<(), GraceErrorKind>
where
    T: Logos<'a, Source = str, Error = GraceErrorKind, Extras = LexerExtras>,
{
    let text = lex.slice();
    Err(GraceErrorKind::SyntaxError(format!(
        "Strings made with double quotes aren't actually treated as strings, use single quotes instead",
    )))
}
