use std::fmt::Debug;

use logos::Logos;

use crate::api::errors::kind::GraceErrorKind;
use crate::api::lexers::{LexerExtras, LexingError, Token};

pub mod java;
pub mod rust;
pub mod sql;
