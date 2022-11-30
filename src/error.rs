use std::{
    error::Error,
    fmt::{Debug, Display},
    io,
};

///Error type for smolldb
#[derive(Debug)]
pub enum SmollError {
    CompressionError(yazi::Error),
    DecodeError,
    FileError(io::Error),
}

impl From<io::Error> for SmollError {
    fn from(value: io::Error) -> Self {
        Self::FileError(value)
    }
}

impl From<yazi::Error> for SmollError {
    fn from(value: yazi::Error) -> Self {
        Self::CompressionError(value)
    }
}

impl Display for SmollError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for SmollError {}

//Result for smolldb
pub type SmollResult<T> = Result<T, SmollError>;
