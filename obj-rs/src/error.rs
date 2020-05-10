//! Contains helper structs for error handling

use std::fmt;
use std::io;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

/// A type for results generated by `load_obj` and `load_mtl` where the `Err` type is hard-wired to
/// `ObjError`
///
/// This typedef is generally used to avoid writing out `ObjError` directly and is otherwise a
/// direct mapping to `std::result::Result`.
pub type ObjResult<T> = Result<T, ObjError>;

/// The error type for loading of the `obj` file.
#[derive(Error, Debug)]
pub enum ObjError {
    /// IO error has been occurred during opening the `obj` file.
    // TODO
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Tried to parse integer frome the `obj` file, but failed.
    // TODO
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    /// Tried to parse floating point number frome the `obj` file, but failed.
    // TODO
    #[error(transparent)]
    ParseFloat(#[from] ParseFloatError),

    /// `LoadError` has been occurred during parseing the `obj` file.
    // TODO
    #[error(transparent)]
    Load(#[from] LoadError),
}

/// The error type for parse operations of the `Obj` struct.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LoadError {
    kind: LoadErrorKind,
    message: &'static str,
}

impl LoadError {
    /// Outputs the detailed cause of loading an OBJ failing.
    pub fn kind(&self) -> &LoadErrorKind {
        &self.kind
    }
}

/// Enum to store the various types of errors that can cause loading an OBJ to fail.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LoadErrorKind {
    /// Met unexpected statement.
    UnexpectedStatement,
    /// Received wrong number of arguments.
    WrongNumberOfArguments,
    /// Received unexpected type of arguments.
    WrongTypeOfArguments,
    /// Model should be triangulated first to be loaded properly.
    UntriangulatedModel,
    /// Model cannot be transformed into requested form.
    InsufficientData,
    /// Received index value out of range.
    IndexOutOfRange,
    /// A line is expected after the backslash (\).
    BackslashAtEOF,
    /// Group number exceeded limitation.
    TooBigGroupNumber,
}

impl LoadError {
    /// Creates a new custom error from a specified kind and message.
    pub fn new(kind: LoadErrorKind, message: &'static str) -> Self {
        LoadError { kind, message }
    }
}

impl std::error::Error for LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LoadErrorKind::*;

        let msg = match self.kind {
            UnexpectedStatement => "Met unexpected statement",
            WrongNumberOfArguments => "Received wrong number of arguments",
            WrongTypeOfArguments => "Received unexpected type of arguments",
            UntriangulatedModel => "Model should be triangulated first to be loaded properly",
            InsufficientData => "Model cannot be transformed into requested form",
            IndexOutOfRange => "Received index value out of range",
            BackslashAtEOF => r"A line is expected after the backslash (\)",
            TooBigGroupNumber => "Group number exceeded limitation.",
        };

        write!(fmt, "{}: {}", msg, self.message)
    }
}

macro_rules! make_error {
    ($kind:ident, $message:expr) => {
        return Err(::std::convert::From::from($crate::error::LoadError::new(
            $crate::error::LoadErrorKind::$kind,
            $message,
        )))
    };
}

pub(crate) use make_error;
