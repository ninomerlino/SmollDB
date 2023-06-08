use std::{
    error,
    fmt::{Debug, Display},
    io, result,
};

use crate::DataType;

///Error type for smolldb
#[derive(Debug)]
pub enum Error {
    CompressionError(yazi::Error),
    DecodeError,
    FileError(io::Error),
    ConversionError(DataType),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::FileError(value)
    }
}

impl From<yazi::Error> for Error {
    fn from(value: yazi::Error) -> Self {
        Self::CompressionError(value)
    }
}


impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl error::Error for Error {}

//Result for smolldb
pub type Result<T> = result::Result<T, Error>;
