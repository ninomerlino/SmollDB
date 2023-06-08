use std::fmt::{Debug, Display};

use crate::Error;

impl From<bool> for DataType {
    fn from(value: bool) -> Self {
        Self::BOOL(value)
    }
}

impl From<i8> for DataType {
    fn from(value: i8) -> Self {
        Self::INT8(value)
    }
}

impl From<i16> for DataType {
    fn from(value: i16) -> Self {
        Self::INT16(value)
    }
}

impl From<i32> for DataType {
    fn from(value: i32) -> Self {
        Self::INT32(value)
    }
}

impl From<i64> for DataType {
    fn from(value: i64) -> Self {
        Self::INT64(value)
    }
}

impl From<f32> for DataType {
    fn from(value: f32) -> Self {
        Self::FLOAT32(value)
    }
}

impl From<f64> for DataType {
    fn from(value: f64) -> Self {
        Self::FLOAT64(value)
    }
}

impl From<String> for DataType {
    fn from(value: String) -> Self {
        Self::STRING(value)
    }
}

impl From<Vec<u8>> for DataType {
    fn from(value: Vec<u8>) -> Self {
        Self::BYTES(value)
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
#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
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

impl Display for DataType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::BOOL(value) => write!(f, "{}", value),
            DataType::INT8(value) => write!(f, "{}", value),
            DataType::INT16(value) => write!(f, "{}", value),
            DataType::INT32(value) => write!(f, "{}", value),
            DataType::INT64(value) => write!(f, "{}", value),
            DataType::FLOAT32(value) => write!(f, "{}", value),
            DataType::FLOAT64(value) => write!(f, "{}", value),
            DataType::STRING(value) => write!(f, "{}", value),
            DataType::BYTES(value) => write!(f, "{:?}", value),
        }
    }
}

impl DataType {
    ///get the id of the current type stored in this Datatype obj
    ///*used for the encoding step*
    pub(crate) fn id(&self) -> u8 {
        match self {
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

impl<'c> TryFrom<&'c DataType> for &'c bool {
    type Error = Error;
    fn try_from(value: &'c DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::BOOL(inner_value) => Ok(inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl<'c> TryFrom<&'c DataType> for &'c i8 {
    type Error = Error;
    fn try_from(value: &'c DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::INT8(inner_value) => Ok(inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl<'c> TryFrom<&'c DataType> for &'c i16 {
    type Error = Error;
    fn try_from(value: &'c DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::INT16(inner_value) => Ok(inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl<'c> TryFrom<&'c DataType> for &'c i32 {
    type Error = Error;
    fn try_from(value: &'c DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::INT32(inner_value) => Ok(inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl<'c> TryFrom<&'c DataType> for &'c i64 {
    type Error = Error;
    fn try_from(value: &'c DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::INT64(inner_value) => Ok(inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl<'c> TryFrom<&'c DataType> for &'c f32 {
    type Error = Error;
    fn try_from(value: &'c DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::FLOAT32(inner_value) => Ok(inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl<'c> TryFrom<&'c DataType> for &'c f64 {
    type Error = Error;
    fn try_from(value: &'c DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::FLOAT64(inner_value) => Ok(inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl<'c> TryFrom<&'c DataType> for &'c String {
    type Error = Error;
    fn try_from(value: &'c DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::STRING(inner_value) => Ok(inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl<'c> TryFrom<&'c DataType> for &'c Vec<u8> {
    type Error = Error;
    fn try_from(value: &'c DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::BYTES(inner_value) => Ok(inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl TryFrom<&DataType> for bool {
    type Error = Error;
    fn try_from(value: &DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::BOOL(inner_value) => Ok(*inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl TryFrom<&DataType> for i8 {
    type Error = Error;
    fn try_from(value: &DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::INT8(inner_value) => Ok(*inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl TryFrom<&DataType> for i16 {
    type Error = Error;
    fn try_from(value: &DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::INT16(inner_value) => Ok(*inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl TryFrom<&DataType> for i32 {
    type Error = Error;
    fn try_from(value: &DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::INT32(inner_value) => Ok(*inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl TryFrom<&DataType> for i64 {
    type Error = Error;
    fn try_from(value: &DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::INT64(inner_value) => Ok(*inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl TryFrom<&DataType> for f32 {
    type Error = Error;
    fn try_from(value: &DataType) -> Result<Self, Error> {
        match value {
            DataType::FLOAT32(inner_value) => Ok(*inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl TryFrom<&DataType> for f64 {
    type Error = Error;
    fn try_from(value: &DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::FLOAT64(inner_value) => Ok(*inner_value),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl TryFrom<&DataType> for String {
    type Error = Error;
    fn try_from(value: &DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::STRING(inner_value) => Ok(inner_value.clone()),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

impl TryFrom<&DataType> for Vec<u8> {
    type Error = Error;
    fn try_from(value: &DataType) -> Result<Self, Self::Error> {
        match value {
            DataType::BYTES(inner_value) => Ok(inner_value.clone()),
            _ => Err(Error::ConversionError(value.clone())),
        }
    }
}

