use std::ops::Sub;

use oak_core::OakErrorKind;

#[derive(Debug, Copy, Clone)]
pub enum Grade {
    Again = 1, // will not compile (mistake is easily known)
    Hard = 2,  // will not compile (mistake is not easily known)
    Good = 3,  // close enough (can still be parsed without much error)
    Easy = 4,  // no mistakes
}

impl Sub<usize> for Grade {
    type Output = isize;

    fn sub(self, rhs: usize) -> Self::Output {
        (self as isize).wrapping_sub_unsigned(rhs)
    }
}

impl From<&OakErrorKind> for Grade {
    fn from(value: &OakErrorKind) -> Self {
        match value {
            OakErrorKind::SyntaxError { .. } => Grade::Again,
            OakErrorKind::UnexpectedCharacter { .. } => Grade::Hard,
            OakErrorKind::UnexpectedToken { .. } => Grade::Hard,
            OakErrorKind::UnexpectedEof { .. } => Grade::Hard,
            OakErrorKind::CustomError { .. } => Grade::Again,
            OakErrorKind::ExpectedToken { .. } => Grade::Good,
            OakErrorKind::ExpectedName { .. } => Grade::Good,
            OakErrorKind::TrailingCommaNotAllowed { .. } => Grade::Good,
            OakErrorKind::ParseError { .. } => Grade::Again,

            // not used
            OakErrorKind::IoError { .. } => Grade::Again,
            OakErrorKind::XmlError { .. } => Grade::Again,
            OakErrorKind::InvalidTheme { .. } => Grade::Again,
            OakErrorKind::UnsupportedFormat { .. } => Grade::Again,
            OakErrorKind::ColorParseError { .. } => Grade::Again,
            OakErrorKind::FormatError { .. } => Grade::Again,
            OakErrorKind::SemanticError { .. } => Grade::Again,
            OakErrorKind::ProtocolError { .. } => Grade::Again,
            OakErrorKind::TestFailure { .. } => Grade::Again,
            OakErrorKind::TestRegenerated { .. } => Grade::Again,
            OakErrorKind::SerdeError { .. } => Grade::Again,
            OakErrorKind::DeserializeError { .. } => Grade::Again,
            OakErrorKind::ZipError { .. } => Grade::Again,
            OakErrorKind::InternalError { .. } => Grade::Again,
        }
    }
}
