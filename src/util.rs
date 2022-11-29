use std::{fmt::{Display,Debug}, error::Error, io::{BufRead}};

impl Into<DataType> for bool {
    fn into(self) -> DataType {
        DataType::BOOL(self)
    }
}

impl Into<DataType> for i8 {
    fn into(self) -> DataType {
        DataType::INT8(self)
    }
}

impl Into<DataType> for i16 {
    fn into(self) -> DataType {
        DataType::INT16(self)
    }
}

impl Into<DataType> for i32 {
    fn into(self) -> DataType {
        DataType::INT32(self)
    }
}

impl Into<DataType> for i64 {
    fn into(self) -> DataType {
        DataType::INT64(self)
    }
}

impl Into<DataType> for f32 {
    fn into(self) -> DataType {
        DataType::FLOAT32(self)
    }
}

impl Into<DataType> for f64 {
    fn into(self) -> DataType {
        DataType::FLOAT64(self)
    }
}

impl Into<DataType> for String {
    fn into(self) -> DataType {
        DataType::STRING(self.to_owned())
    }
}

impl Into<DataType> for Vec<u8> {
    fn into(self) -> DataType {
        DataType::BYTES(self.to_owned())
    }
}

///A union for all the types supported by smolldb
///
///Supported types are:
///* `bool`
///* `i8`
///* `i16` 
///* `i32`
///* `i64`
///* `f32`
///* `f64`
///* `String`
///* `Vec<u8>`
pub enum DataType{
    BOOL(bool),
    INT8(i8),
    INT16(i16),
    INT32(i32),
    INT64(i64),
    FLOAT32(f32),
    FLOAT64(f64),
    STRING(String),
    BYTES(Vec<u8>),
}

impl DataType {
    ///get the id of the current type stored in this Datatype obj
    ///*used for the encoding step*
    pub(crate) fn id(&self) -> u8{
        match self{
            DataType::BOOL(_) => 0,
            DataType::INT8(_) => 1,
            DataType::INT16(_) => 2,
            DataType::INT32(_) => 3,
            DataType::INT64(_) => 4,
            DataType::FLOAT32(_) => 5,
            DataType::FLOAT64(_) => 6,
            DataType::STRING(_) => 7,
            DataType::BYTES(_) => 8,
        }
    }
}

impl PartialEq<DataType> for DataType {
    fn eq(&self, other: &DataType) -> bool {
        match (self, other) {
            (Self::BOOL(l0), Self::BOOL(r0)) => l0 == r0,
            (Self::INT8(l0), Self::INT8(r0)) => l0 == r0,
            (Self::INT16(l0), Self::INT16(r0)) => l0 == r0,
            (Self::INT32(l0), Self::INT32(r0)) => l0 == r0,
            (Self::INT64(l0), Self::INT64(r0)) => l0 == r0,
            (Self::FLOAT32(l0), Self::FLOAT32(r0)) => l0 == r0,
            (Self::FLOAT64(l0), Self::FLOAT64(r0)) => l0 == r0,
            (Self::STRING(l0), Self::STRING(r0)) => l0 == r0,
            (Self::BYTES(l0), Self::BYTES(r0)) => l0 == r0,
            (_,_) => false,
        }
    }
}

impl Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BOOL(arg0) => f.debug_tuple("BOOL").field(arg0).finish(),
            Self::INT8(arg0) => f.debug_tuple("INT8").field(arg0).finish(),
            Self::INT16(arg0) => f.debug_tuple("INT16").field(arg0).finish(),
            Self::INT32(arg0) => f.debug_tuple("INT32").field(arg0).finish(),
            Self::INT64(arg0) => f.debug_tuple("INT64").field(arg0).finish(),
            Self::FLOAT32(arg0) => f.debug_tuple("FLOAT32").field(arg0).finish(),
            Self::FLOAT64(arg0) => f.debug_tuple("FLOAT64").field(arg0).finish(),
            Self::STRING(arg0) => f.debug_tuple("STRING").field(arg0).finish(),
            Self::BYTES(arg0) => f.debug_tuple("BYTES").field(arg0).finish(),
        }
    }
}

///Error type for smoll db
pub enum SmollError{
    InsertError,
    CompressionError,
    DecompressionError,
    DecodeError,
    SaveFileError,
    ReadFileError,
}

impl SmollError {
    fn string(&self) -> String {
        match self {
            SmollError::InsertError => String::from("Insert Error"),
            SmollError::CompressionError => String::from("Compression Error"),
            SmollError::DecompressionError => String::from("Decompression Error"),
            SmollError::SaveFileError => String::from("Save File Error"),
            SmollError::ReadFileError => String::from("Read File Error"),
            SmollError::DecodeError => String::from("Decode Error"),
        }
    }
}

impl Display for SmollError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.string())
    }
}

impl Debug for SmollError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.string())
    }
}

impl Error for SmollError {}
